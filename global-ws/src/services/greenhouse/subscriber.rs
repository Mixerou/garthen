use actix_web_actors::ws::WebsocketContext;

use crate::error::{WebSocketError, WebSocketErrorTemplate};
use crate::messages::{DispatchEvent, DispatchMessage, WebSocketMessage, WebSocketMessageData};
use crate::server::{Socket, WebSocketConnection};
use crate::services::greenhouse::Greenhouse;
use crate::services::session::Session;

fn greenhouse_update(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    let WebSocketMessageData::SubscribeToGreenhouseUpdate { id: greenhouse_id }
        = message.data else { return Err(WebSocketErrorTemplate::BadRequest(None).into()) };

    let session = Session::find(connection.session_id.unwrap())?;
    let Some(session_user_id)
        = session.user_id else { return Err(WebSocketErrorTemplate::Unauthorized(None).into()) };
    let greenhouse
        = Greenhouse::find_by_id_and_owner_id(greenhouse_id, session_user_id)?;

    let response = DispatchMessage {
        event: DispatchEvent::GreenhouseUpdate { id: greenhouse.id },
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

fn greenhouses_mine_update(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    let session = Session::find(connection.session_id.unwrap())?;

    let Some(session_user_id)
        = session.user_id else { return Err(WebSocketErrorTemplate::Unauthorized(None).into()) };
    let greenhouses = Greenhouse::find_all_by_owner_id(session_user_id)?;

    for greenhouse in greenhouses {
        let response = DispatchMessage {
            event: DispatchEvent::GreenhouseUpdate { id: greenhouse.id },
            new_subscribers: Some(vec![connection.id]),
        };

        Socket::send_message(
            message.id,
            response,
            connection.socket.downgrade().recipient(),
            connection,
            context,
        )?;
    }

    Ok(())
}

fn greenhouse_create(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    let session = Session::find(connection.session_id.unwrap())?;
    let Some(session_user_id)
        = session.user_id else { return Err(WebSocketErrorTemplate::Unauthorized(None).into()) };

    let response = DispatchMessage {
        event: DispatchEvent::GreenhouseCreate { id: None, owner_id: session_user_id },
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

fn greenhouse_delete(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    let session = Session::find(connection.session_id.unwrap())?;
    let Some(session_user_id)
        = session.user_id else { return Err(WebSocketErrorTemplate::Unauthorized(None).into()) };

    let response = DispatchMessage {
        event: DispatchEvent::GreenhouseDelete { id: None, owner_id: session_user_id },
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
        "greenhouse" => greenhouse_update(message, connection, context)?,
        "greenhouses/mine" => greenhouses_mine_update(message, connection, context)?,
        "greenhouse-create" => greenhouse_create(message, connection, context)?,
        "greenhouse-delete" => greenhouse_delete(message, connection, context)?,
        _ => return Err(WebSocketErrorTemplate::InvalidRequestField(None).into()),
    }

    Ok(())
}
