use std::borrow::Borrow;
use std::collections::HashMap;
use std::time::Instant;

use actix::{ActorFutureExt, AsyncContext, ContextFutureSpawner, Message, WeakRecipient, WrapFuture};
use actix::prelude::{Actor, Context, Handler, Recipient};
use actix_web_actors::ws::WebsocketContext;

use crate::error::WebSocketError;
use crate::messages::{AuthorizationMessage, DisconnectionMessage, Opcode, WebSocketMessage, WebSocketMessageData};
use crate::server::WebSocketConnection;

#[derive(Debug, Default)]
pub struct Socket {
    connections: HashMap<i64, Recipient<WebSocketMessage>>,
}

impl Socket {
    pub fn handle(
        connection: &mut WebSocketConnection,
        message: WebSocketMessage,
        context: &mut WebsocketContext<WebSocketConnection>,
    ) -> Result<(), WebSocketError> {
        let address = connection.address.downgrade();
        let message_id = message.id;

        if connection.session_id.is_none() || message.opcode == Opcode::Authorize {
            if message.opcode != Opcode::Authorize {
                return Err(WebSocketError::new(
                    401,
                    "Unauthorized".to_string(),
                    None)
                );
            }

            let data = match message.data {
                Some(message) => message,
                None => {
                    return Err(WebSocketError::new(
                        400,
                        "Bad request".to_string(),
                        None)
                    );
                }
            };

            let token = match data.token {
                Some(token) => token,
                None => {
                    return Err(WebSocketError::new(
                        400,
                        "Bad request".to_string(),
                        None)
                    );
                }
            };

            let authorization_message = AuthorizationMessage {
                id: message.id,
                connection_id: connection.id,
                token,
                address: context.address().recipient(),
            };

            Socket::send_message(
                message_id,
                authorization_message,
                address.recipient(),
                connection,
                context,
            )?;

            // TODO Set session_id from DB
            connection.session_id = Some(789);

            return Ok(());
        }

        match message.opcode {
            Opcode::HeartBeat => {
                connection.last_heartbeat_at = Instant::now();

                let response = WebSocketMessage {
                    id: message.id,
                    connection_id: message.connection_id,
                    opcode: Opcode::Response,
                    method: None,
                    request: None,
                    data: None,
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
            Opcode::Authorize => {}
            Opcode::Dispatch | Opcode::Error => {
                return Err(WebSocketError::new(
                    400,
                    "Opcode not allowed".to_string(),
                    None)
                );
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
                        method: None,
                        request: None,
                        data: Some(WebSocketMessageData {
                            code: Some(error.status_code),
                            message: Some(error.get_safe_message()),
                            ..Default::default()
                        }),
                    });
                }
            })
            .spawn(context);

        Ok(())
    }

    fn get_connection(&self, id: &i64) -> Result<&Recipient<WebSocketMessage>, WebSocketError> {
        match self.connections.get(id) {
            Some(connection) => Ok(connection),
            None => {
                Err(WebSocketError::new(
                    500,
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
        let connection = Socket::get_connection(
            self.borrow(),
            &message.connection_id,
        )?;

        connection.do_send(message);

        Ok(())
    }
}

impl Handler<AuthorizationMessage> for Socket {
    type Result = Result<(), WebSocketError>;

    fn handle(&mut self, message: AuthorizationMessage, _: &mut Context<Self>) -> Self::Result {
        // TODO Check token in DB

        self.connections.insert(
            message.connection_id,
            message.address,
        );

        let connection = Socket::get_connection(
            self.borrow(),
            &message.connection_id,
        )?;

        connection.do_send(WebSocketMessage {
            id: message.id,
            connection_id: message.connection_id,
            opcode: Opcode::Response,
            method: None,
            request: None,
            data: Some(WebSocketMessageData {
                code: Some(200),
                message: Some("Successfully authorized".to_string()),
                ..Default::default()
            }),
        });

        Ok(())
    }
}

impl Handler<DisconnectionMessage> for Socket {
    type Result = Result<(), WebSocketError>;

    fn handle(&mut self, message: DisconnectionMessage, _: &mut Context<Self>) -> Self::Result {
        self.connections.remove(&message.connection_id);

        Ok(())
    }
}