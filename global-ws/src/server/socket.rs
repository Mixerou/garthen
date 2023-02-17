use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

use actix::{ActorFutureExt, AsyncContext, ContextFutureSpawner, Message, WeakRecipient, WrapFuture};
use actix::prelude::{Actor, Context, Handler, Recipient};
use actix_web_actors::ws::WebsocketContext;

use crate::error::{WebSocketCloseError, WebSocketError, WebSocketErrorTemplate};
use crate::messages::{AuthorizationMessage, DisconnectionMessage, DispatchEvent, DispatchMessage, Opcode, WebSocketMessage, WebSocketMessageData};
use crate::server::WebSocketConnection;
use crate::services::session::Session;
use crate::services::user;
use crate::services::user::{User, UserMe};

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
        let address = connection.address.downgrade();
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
                address.recipient(),
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
                    address.recipient(),
                    connection,
                    context,
                )?;
            }
            Opcode::Request => {}
            Opcode::Response => {}
            Opcode::Authorize => {
                Socket::close_connection(WebSocketCloseError::AlreadyAuthenticated, context);
            },
            Opcode::Subscribe => {
                let request = match message.request.to_owned() {
                    Some(request) => request,
                    None => return Err(WebSocketErrorTemplate::BadRequest(None).into()),
                };

                match request.as_str() {
                    "user" => user::subscribe(request, message, connection, context)?,
                    "user/me" => user::subscribe(request, message, connection, context)?,
                    _ => {
                        return Err(WebSocketErrorTemplate::BadRequest(None).into());
                    },
                }
            },
            Opcode::Dispatch | Opcode::Error => {
                Socket::close_connection(WebSocketCloseError::Opcode, context);
            }
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
        let subscribers = self.subscriptions.entry(message.event.to_owned())
            .or_insert(HashSet::new());
        let new_subscribers = message.new_subscribers.unwrap_or(vec![]);

        let data: WebSocketMessageData = match message.event {
            DispatchEvent::UserUpdate { id } => {
                WebSocketMessageData::from(User::find(id)?)
            },
            DispatchEvent::UserMeUpdate { id } => {
                WebSocketMessageData::from(UserMe::from(User::find(id)?))
            },
        };

        match new_subscribers {
            new_subscribers if new_subscribers.is_empty() => {
                let subscribers_vec = subscribers.iter();

                for subscriber_id in subscribers_vec {
                    let message = WebSocketMessage {
                        id: snowflake::generate(),
                        connection_id: subscriber_id.to_owned(),
                        opcode: Opcode::Dispatch,
                        event: Some(message.event.to_owned()),
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
                        event: Some(message.event.to_owned()),
                        data: data.to_owned(),
                        ..Default::default()
                    };

                    if let Some((connection, subscriptions))
                        = self.connections.get_mut(&subscriber_id) {
                        subscriptions.insert(message.event.to_owned());
                        connection.do_send(response);
                    }
                }
            },
        };

        Ok(())
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
