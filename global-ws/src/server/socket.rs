use std::borrow::Borrow;
use std::collections::HashMap;
use std::time::Instant;

use actix::{ActorFutureExt, AsyncContext, ContextFutureSpawner, Message, WeakRecipient, WrapFuture};
use actix::prelude::{Actor, Context, Handler, Recipient};
use actix_web_actors::ws::WebsocketContext;

use crate::error::{WebSocketCloseError, WebSocketError, WebSocketErrorTemplate};
use crate::messages::{AuthorizationMessage, DisconnectionMessage, Opcode, WebSocketMessage, WebSocketMessageData};
use crate::server::WebSocketConnection;
use crate::services::session::Session;

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


        if connection.session_id.is_none() {
            if message.opcode != Opcode::Authorize {
                Socket::close_connection(WebSocketCloseError::NotAuthenticated, context);

                return Ok(())
            }

            let data = match message.data {
                Some(message) => message,
                None => return Err(WebSocketErrorTemplate::BadRequest(None).into()),
            };

            let token = match data.token {
                Some(token) => token,
                None => return Err(WebSocketErrorTemplate::BadRequest(None).into()),
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
            Opcode::Authorize => {
                Socket::close_connection(WebSocketCloseError::AlreadyAuthenticated, context);
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
                        method: None,
                        request: None,
                        data: Some(WebSocketMessageData {
                            code: Some(error.json_code),
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
