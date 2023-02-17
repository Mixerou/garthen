use std::sync::Arc;
use std::time::{Duration, Instant};

use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, Running, StreamHandler};
use actix_web_actors::ws;
use actix_web_actors::ws::{CloseCode, CloseReason, ProtocolError, WebsocketContext};
use serde::{Deserialize, Serialize};

use crate::error::{WebSocketCloseError, WebSocketError};
use crate::messages::{DisconnectionMessage, Opcode, WebSocketMessage, WebSocketMessageData};
use crate::server::Socket;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(45);

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Encoding {
    Etf,
    Json,
}

#[derive(Debug)]
pub struct WebSocketConnection {
    pub id: i64,
    pub session_id: Option<i64>,
    pub last_heartbeat_at: Instant,
    pub encoding: Encoding,
    pub address: Arc<Addr<Socket>>,
}

impl WebSocketConnection {
    pub fn new(encoding: Encoding, address: Arc<Addr<Socket>>) -> Self {
        WebSocketConnection {
            id: snowflake::generate(),
            session_id: None,
            last_heartbeat_at: Instant::now(),
            encoding,
            address,
        }
    }

    fn heartbeat(&self, ctx: &mut WebsocketContext<Self>) {
        ctx.run_interval(
            HEARTBEAT_INTERVAL,
            |actor, ctx| {
                if Instant::now().duration_since(actor.last_heartbeat_at) > CLIENT_TIMEOUT {
                    let close_reason = CloseReason {
                        code: CloseCode::Normal,
                        description: None,
                    };

                    ctx.close(Some(close_reason));
                    ctx.stop();
                }
            },
        );
    }

    fn send_message(
        encoding: Encoding,
        message: WebSocketMessage,
        context: &mut WebsocketContext<WebSocketConnection>,
    ) -> Result<(), WebSocketError> {
        match encoding {
            Encoding::Etf => {
                let bytes = serde_eetf::to_bytes(&message)?;

                context.binary(bytes);
            }
            Encoding::Json => {
                context.text(serde_json::to_string(&message)?);
            }
        };

        Ok(())
    }

    fn handle_message(
        &mut self,
        message: WebSocketMessage,
        context: &mut WebsocketContext<WebSocketConnection>,
    ) {
        let message = WebSocketMessage {
            connection_id: self.id,
            ..message
        };

        let id = message.id;
        let connection_id = message.connection_id;

        match Socket::handle(self, message, context) {
            Ok(_) => {}
            Err(error) => {
                WebSocketConnection::send_message(
                    self.encoding,
                    WebSocketMessage {
                        id,
                        connection_id,
                        opcode: Opcode::Error,
                        data: Some(WebSocketMessageData {
                            code: Some(error.json_code),
                            message: Some(error.get_safe_message()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    context,
                ).unwrap()
            }
        };
    }
}

impl Actor for WebSocketConnection {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, context: &mut Self::Context) {
        self.heartbeat(context);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.address.do_send(DisconnectionMessage {
            connection_id: self.id,
        });

        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ProtocolError>> for WebSocketConnection {
    fn handle(
        &mut self,
        message: Result<ws::Message, ProtocolError>,
        context: &mut Self::Context,
    ) {
        let message = match message {
            Ok(message) => message,
            Err(_) => {
                context.stop();
                return;
            }
        };

        match message {
            ws::Message::Text(message) => {
                if self.encoding != Encoding::Json {
                    Socket::close_connection(WebSocketCloseError::InvalidPayload, context);
                }

                let message: WebSocketMessage = match serde_json::from_str(&message) {
                    Ok(message) => message,
                    Err(_) => {
                        Socket::close_connection(
                            WebSocketCloseError::InvalidPayload,
                            context,
                        );

                        return;
                    }
                };

                WebSocketConnection::handle_message(self, message, context);
            }
            ws::Message::Binary(message) => {
                if self.encoding != Encoding::Etf {
                    Socket::close_connection(WebSocketCloseError::InvalidPayload, context);
                }

                let message: WebSocketMessage = match serde_eetf::from_bytes(&message) {
                    Ok(message) => message,
                    Err(_) => {
                        Socket::close_connection(
                            WebSocketCloseError::InvalidPayload,
                            context,
                        );

                        return;
                    }
                };

                WebSocketConnection::handle_message(self, message, context);
            }
            ws::Message::Close(reason) => {
                context.close(reason);
                context.stop();
            }
            _ => (),
        }
    }
}

impl Handler<WebSocketMessage> for WebSocketConnection {
    type Result = Result<(), WebSocketError>;

    fn handle(&mut self, message: WebSocketMessage, context: &mut Self::Context) -> Self::Result {
        WebSocketConnection::send_message(self.encoding, message, context)?;

        Ok(())
    }
}
