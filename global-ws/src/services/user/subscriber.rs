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
    let (id, is_me) = match message.data {
        WebSocketMessageData::SubscribeToUserUpdates { id, is_me} => (id, is_me),
        _ => return Err(WebSocketErrorTemplate::BadRequest(None).into()),
    };

    if id.is_none() && !is_me.unwrap_or(false) {
        return Err(WebSocketErrorTemplate::BadRequest(None).into());
    }

    let mut user_id: i64 = 0;
    let session = Session::find(connection.session_id.unwrap())?;
    let session_user_id = match session.user_id {
        Some(user_id) => user_id,
        None => return Err(WebSocketErrorTemplate::Unauthorized(None).into()),
    };

    if let Some(id) = id {
        user_id = id;
    }

    if is_me.unwrap_or(false) {
        user_id = session_user_id;
    }

    let response = DispatchMessage {
        event: DispatchEvent::UserUpdate { id: user_id },
        new_subscribers: Some(vec![connection.id]),
    };

    Socket::send_message(
        message.id,
        response,
        connection.address.downgrade().recipient(),
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

    let session_user_id = match session.user_id {
        Some(user_id) => user_id,
        None => return Err(WebSocketErrorTemplate::Unauthorized(None).into()),
    };

    let response = DispatchMessage {
        event: DispatchEvent::UserMeUpdate { id: session_user_id },
        new_subscribers: Some(vec![connection.id]),
    };

    Socket::send_message(
        message.id,
        response,
        connection.address.downgrade().recipient(),
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
