use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use actix::{ActorFutureExt, AsyncContext, ContextFutureSpawner, Message, WeakRecipient, WrapFuture};
use actix::prelude::{Actor, Context, Handler, Recipient};
use actix_broker::{BrokerIssue, BrokerSubscribe};
use actix_web_actors::ws::WebsocketContext;

use crate::error::{WebSocketCloseError, WebSocketError, WebSocketErrorTemplate};
use crate::messages::{AmqpPayload, AuthorizationMessage, DisconnectionMessage, DispatchAmqpMessage, DispatchEvent, DispatchMessage, InitAmqpConsumersMessage, Opcode, WebSocketMessage, WebSocketMessageData};
use crate::server::WebSocketConnection;
use crate::services::{device, device_record, greenhouse, user};
use crate::services::device::Device;
use crate::services::device_record::{DeviceRecord, DeviceRecordsAverage, DeviceRecordsTimestampRange};
use crate::services::greenhouse::Greenhouse;
use crate::services::session::Session;
use crate::services::user::{UserMe, UserPublic};

const MINUTE_AS_SECS: u64 = 60;
const HOUR_AS_SECS: u64 = MINUTE_AS_SECS * 60;
const DAY_AS_SECS: u64 = HOUR_AS_SECS * 24;
const WEEK_AS_SECS: u64 = DAY_AS_SECS * 7;
const MONTH_AS_SECS: u64 = 365 / 12 * DAY_AS_SECS;

#[derive(Debug, Default)]
pub struct Socket {
    connections: HashMap<i64, (Recipient<WebSocketMessage>, HashSet<DispatchEvent>)>,
    subscriptions: HashMap<DispatchEvent, HashSet<i64>>,
}

impl Socket {
    pub fn handle(
        connection: &mut WebSocketConnection,
        message: WebSocketMessage,
        context: &mut WebsocketContext<WebSocketConnection>,
    ) -> Result<(), WebSocketError> {
        let socket = connection.socket.downgrade();
        let message_id = message.id;


        if connection.session_id.is_none() {
            if message.opcode != Opcode::Authorize {
                Socket::close_connection(WebSocketCloseError::NotAuthenticated, context);

                return Ok(())
            }

            let token = match message.data {
                WebSocketMessageData::Authorize { token } => token,
                _ => return Err(WebSocketErrorTemplate::BadRequest(None).into()),
            };

            let session = match Session::find_by_token(token.to_owned()) {
                Ok(session) => session,
                Err(error) => return match error.http_code {
                    404 => {
                        Socket::close_connection(
                            WebSocketCloseError::AuthenticationFailed,
                            context,
                        );

                        Ok(())
                    },
                    _ => Err(error),
                }
            };

            if session.user_id.is_none() {
                return Err(WebSocketErrorTemplate::Unauthorized(None).into());
            }

            let authorization_message = AuthorizationMessage {
                id: message.id,
                connection_id: connection.id,
                token,
                address: context.address().recipient(),
            };

            connection.session_id = Some(session.id);

            Socket::send_message(
                message_id,
                authorization_message,
                socket.recipient(),
                connection,
                context,
            )?;

            return Ok(());
        }

        match message.opcode {
            Opcode::HeartBeat => {
                connection.last_heartbeat_at = Instant::now();

                let response = WebSocketMessage {
                    id: message.id,
                    connection_id: message.connection_id,
                    opcode: Opcode::Response,
                    ..Default::default()
                };


                Socket::send_message(
                    message_id,
                    response,
                    socket.recipient(),
                    connection,
                    context,
                )?;
            }
            Opcode::Request => {
                let (Some(request), Some(method))
                    = (message.request.to_owned(), message.method.to_owned()) else {
                    return Err(WebSocketErrorTemplate::BadRequest(None).into())
                };

                let handle = match request.as_str() {
                    "user/me" => user::handle,
                    "greenhouse" => greenhouse::handle,
                    "device"
                    | "device/state"
                    | "device/custom-data"
                    | "device/request-data"
                    | "device/disable"
                    | "device/enable"
                    | "devices/reset-names" => device::handle,
                    _ => return Err(WebSocketErrorTemplate::BadRequest(None).into()),
                };

                handle(request, method, message, connection, context)?;
            },
            Opcode::Response => {}
            Opcode::Authorize =>
                Socket::close_connection(WebSocketCloseError::AlreadyAuthenticated, context),
            Opcode::Subscribe => {
                let Some(request) = message.request.to_owned() else {
                    return Err(WebSocketErrorTemplate::BadRequest(None).into())
                };

                let subscribe = match request.as_str() {
                    "user" | "user/me" => user::subscribe,
                    "greenhouse"
                    | "greenhouses/mine"
                    | "greenhouse-create"
                    | "greenhouse-delete" => greenhouse::subscribe,
                    "device" | "devices" => device::subscribe,
                    "device_records" | "device_records/average" => device_record::subscribe,
                    _ => return Err(WebSocketErrorTemplate::BadRequest(None).into()),
                };

                subscribe(request, message, connection, context)?;
            },
            Opcode::Dispatch | Opcode::Error =>
                Socket::close_connection(WebSocketCloseError::Opcode, context),
        }

        Ok(())
    }


    pub fn send_message<T>(
        message_id: i64,
        message: T,
        recipient: WeakRecipient<T>,
        connection: &mut WebSocketConnection,
        context: &mut WebsocketContext<WebSocketConnection>,
    ) -> Result<(), WebSocketError>
        where
            T: Message<Result=Result<(), WebSocketError>> + Send + 'static,
            T::Result: Send {
        let recipient = match recipient.upgrade() {
            Some(recipient) => recipient,
            None => {
                return Err(WebSocketError::new(
                    500,
                    None,
                    "Failed to upgrade recipient in Socket::send".to_string(),
                    None));
            }
        };

        async move { recipient.send(message).await? }
            .into_actor(connection)
            .map(move |result,
                       connection,
                       context,
            | {
                if let Err(error) = result {
                    context.address().do_send(WebSocketMessage {
                        id: message_id,
                        connection_id: connection.id,
                        opcode: Opcode::Error,
                        data: WebSocketMessageData::Response {
                            code: error.json_code,
                            message: error.get_safe_message(),
                        },
                        ..Default::default()
                    });
                }
            })
            .spawn(context);

        Ok(())
    }

    fn get_connection(&self, id: &i64) -> Result<&(Recipient<WebSocketMessage>, HashSet<DispatchEvent>), WebSocketError> {
        match self.connections.get(id) {
            Some(connection) => Ok(connection),
            None => {
                Err(WebSocketError::new(
                    500,
                    None,
                    "Couldn't find a connection".to_string(),
                    None)
                )
            }
        }
    }
}

impl Actor for Socket {
    type Context = Context<Self>;

    fn started(&mut self, context: &mut Self::Context) {
        self.issue_system_async(InitAmqpConsumersMessage(context.address().downgrade()));
        self.subscribe_system_async::<DispatchAmqpMessage>(context);
    }
}

impl Handler<WebSocketMessage> for Socket {
    type Result = Result<(), WebSocketError>;

    fn handle(&mut self, message: WebSocketMessage, _: &mut Context<Self>) -> Self::Result {
        let (connection, _) = Socket::get_connection(
            self.borrow(),
            &message.connection_id,
        )?;

        connection.do_send(message);

        Ok(())
    }
}

impl Handler<DispatchMessage> for Socket {
    type Result = Result<(), WebSocketError>;

    fn handle(&mut self, message: DispatchMessage, _: &mut Context<Self>) -> Self::Result {
        let new_subscribers = message.new_subscribers.unwrap_or(vec![]);
        let mut event = message.event;

        let subscribers = self.subscriptions.entry(event.to_owned())
            .or_insert(HashSet::new());

        if new_subscribers.is_empty() && subscribers.is_empty() { return Ok(()) }

        let data: WebSocketMessageData = match event {
            DispatchEvent::UserUpdate { id } => {
                WebSocketMessageData::from(UserPublic::find(id)?)
            },
            DispatchEvent::UserMeUpdate { id } => {
                WebSocketMessageData::from(UserMe::find(id)?)
            },
            DispatchEvent::GreenhouseUpdate { id } => {
                WebSocketMessageData::from(Greenhouse::find(id)?)
            },
            DispatchEvent::GreenhouseCreate { id, owner_id } => {
                match id {
                    Some(id) => {
                        event = DispatchEvent::GreenhouseCreate { id: None, owner_id };

                        WebSocketMessageData::from(Greenhouse::find(id)?)
                    },
                    None => WebSocketMessageData::None,
                }
            },
            DispatchEvent::GreenhouseDelete { id, owner_id } => {
                match id {
                    Some(id) => {
                        event = DispatchEvent::GreenhouseDelete { id: None, owner_id };

                        WebSocketMessageData::DispatchGreenhouseMineDelete { id }
                    },
                    None => WebSocketMessageData::None,
                }
            },
            DispatchEvent::DeviceUpdate { id } => {
                WebSocketMessageData::from(Device::find(id)?)
            },
            DispatchEvent::DeviceRecordsUpdate { device_id } => {
                WebSocketMessageData::DispatchDeviceRecordsUpdate {
                    device_id,
                    quantity: DeviceRecord::count_by_device_id(device_id)?,
                }
            }
            DispatchEvent::DeviceRecordsAverageUpdate {
                device_id,
                range,
            } => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH).unwrap()
                    .as_secs();
                let mut records = vec![];

                let (now, iterations, interval): (u64, u64, u64) = match range {
                    DeviceRecordsTimestampRange::Today => (now, 24, HOUR_AS_SECS),
                    DeviceRecordsTimestampRange::Week => (now, 7, DAY_AS_SECS),
                    DeviceRecordsTimestampRange::Month => (now, 5, WEEK_AS_SECS),
                    DeviceRecordsTimestampRange::LastMonth =>
                        (now - MONTH_AS_SECS, 5, WEEK_AS_SECS),
                    DeviceRecordsTimestampRange::MonthBeforeLast =>
                        (now - MONTH_AS_SECS * 2, 5, WEEK_AS_SECS),
                    DeviceRecordsTimestampRange::LastThreeMoths => (now, 3, MONTH_AS_SECS),
                };

                for i in 0..iterations {
                    let from = now - interval * (i + 1);
                    let until = now - interval * i;

                    let data
                        = DeviceRecord::get_average_between_timestamp_by_device_id(
                        device_id,
                        (
                            UNIX_EPOCH + Duration::from_secs(from),
                            UNIX_EPOCH + Duration::from_secs(until),
                        ),
                    )?.map(|data| (data * 100.0).trunc() / 100.0);

                    records.push(DeviceRecordsAverage {
                        data,
                        range: (from, until),
                    });
                }

                WebSocketMessageData::DispatchDeviceRecordsAverageUpdate {
                    device_id,
                    range,
                    records,
                }
            }
        };

        match new_subscribers {
            new_subscribers if new_subscribers.is_empty() => {
                let subscribers_vec = subscribers.iter();

                if data.is_none() { return Ok(()) }

                for subscriber_id in subscribers_vec {
                    let message = WebSocketMessage {
                        id: snowflake::generate(),
                        connection_id: subscriber_id.to_owned(),
                        opcode: Opcode::Dispatch,
                        event: Some(event.to_owned()),
                        data: data.to_owned(),
                        ..Default::default()
                    };

                    if let Some((connection, _))
                        = self.connections.get(subscriber_id) {
                        connection.do_send(message);
                    }
                }
            },
            _ => {
                for subscriber_id in new_subscribers {
                    subscribers.insert(subscriber_id);


                    let response = WebSocketMessage {
                        id: snowflake::generate(),
                        connection_id: subscriber_id,
                        opcode: Opcode::Dispatch,
                        event: Some(event.to_owned()),
                        data: data.to_owned(),
                        ..Default::default()
                    };

                    if let Some((connection, subscriptions))
                        = self.connections.get_mut(&subscriber_id) {
                        subscriptions.insert(event.to_owned());

                        if !data.is_none() {
                            connection.do_send(response);
                        }
                    }
                }
            },
        };

        Ok(())
    }
}

impl Handler<DispatchAmqpMessage> for Socket {
    type Result = ();

    fn handle(
        &mut self,
        message: DispatchAmqpMessage,
        context: &mut Self::Context,
    ) -> Self::Result {
        match message.payload {
            AmqpPayload::DispatchData { device_id } => {
                context.address().do_send(DispatchMessage {
                    event: DispatchEvent::DeviceUpdate { id: device_id },
                    new_subscribers: None,
                });
                context.address().do_send(DispatchMessage {
                    event: DispatchEvent::DeviceRecordsUpdate { device_id },
                    new_subscribers: None,
                });

                // TODO: Uncomment when correct time parsing in dispatcher is done
                // let device_records_average_ranges = vec![
                //     DeviceRecordsTimestampRange::Today,
                //     DeviceRecordsTimestampRange::Week,
                //     DeviceRecordsTimestampRange::Month,
                //     DeviceRecordsTimestampRange::LastThreeMoths,
                // ];
                //
                // for range in device_records_average_ranges {
                //     context.address().do_send(DispatchMessage {
                //         event: DispatchEvent::DeviceRecordsAverageUpdate { device_id, range },
                //         new_subscribers: None,
                //     });
                // }
            },
            AmqpPayload::DispatchDevice { id } => {
                context.address().do_send(DispatchMessage {
                    event: DispatchEvent::DeviceUpdate { id },
                    new_subscribers: None,
                });
            },
            AmqpPayload::Ping => debug!("Got Ping from AMQP"),
            _ => {},
        }
    }
}

impl Handler<AuthorizationMessage> for Socket {
    type Result = Result<(), WebSocketError>;

    fn handle(&mut self, message: AuthorizationMessage, _: &mut Context<Self>) -> Self::Result {
        self.connections.insert(
            message.connection_id,
            (message.address, HashSet::new()),
        );

        let (connection, _) = Socket::get_connection(
            self.borrow(),
            &message.connection_id,
        )?;

        connection.do_send(WebSocketMessage {
            id: message.id,
            connection_id: message.connection_id,
            opcode: Opcode::Response,
            data: WebSocketMessageData::Response {
                code: 200,
                message: "Successfully authorized".to_string(),
            },
            ..Default::default()
        });

        Ok(())
    }
}

impl Handler<DisconnectionMessage> for Socket {
    type Result = Result<(), WebSocketError>;

    fn handle(&mut self, message: DisconnectionMessage, _: &mut Context<Self>) -> Self::Result {
        let (_, connection_subscriptions)
            = match self.connections.get(&message.connection_id) {
            Some(connection) => connection,
            None => {
                return Err(WebSocketError::new(
                    500,
                    None,
                    "Couldn't find a connection".to_string(),
                    None)
                )
            }
        };

        for user_subscription in connection_subscriptions.iter() {
            if let Some(subscription)
                = self.subscriptions.get_mut(user_subscription) {
                subscription.remove(&message.connection_id);

                if subscription.is_empty() {
                    self.subscriptions.remove(user_subscription);
                }
            }
        }

        self.connections.remove(&message.connection_id);

        Ok(())
    }
}
