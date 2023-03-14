use actix_web_actors::ws::WebsocketContext;

use crate::error::{WebSocketError, WebSocketErrorTemplate};
use crate::messages::{DispatchEvent, DispatchMessage, WebSocketMessage, WebSocketMessageData};
use crate::server::{Socket, WebSocketConnection};
use crate::services::session::Session;

fn user_update(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    let WebSocketMessageData::SubscribeToUserUpdate { id }
        = message.data else { return Err(WebSocketErrorTemplate::BadRequest(None).into()) };

    let response = DispatchMessage {
        event: DispatchEvent::UserUpdate { id },
        new_subscribers: Some(vec![connection.id]),
    };

    Socket::send_message(
        message.id,
        response,
        connection.socket.downgrade().recipient(),
        connection,
        context,
    )?;

    Ok(())
}

fn user_me_update(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    let session = Session::find(connection.session_id.unwrap())?;
    let Some(session_user_id)
        = session.user_id else { return Err(WebSocketErrorTemplate::Unauthorized(None).into()) };

    let response = DispatchMessage {
        event: DispatchEvent::UserMeUpdate { id: session_user_id },
        new_subscribers: Some(vec![connection.id]),
    };

    Socket::send_message(
        message.id,
        response,
        connection.socket.downgrade().recipient(),
        connection,
        context,
    )?;

    Ok(())
}

pub fn subscribe(
    to: String,
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    match to.as_str() {
        "user" => user_update(message, connection, context)?,
        "user/me" => user_me_update(message, connection, context)?,
        _ => return Err(WebSocketErrorTemplate::InvalidRequestField(None).into()),
    }

    Ok(())
}
