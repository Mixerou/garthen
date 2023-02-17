use actix_web_actors::ws::WebsocketContext;

use crate::error::{WebSocketError, WebSocketErrorTemplate};
use crate::messages::{DispatchEvent, DispatchMessage, WebSocketMessage, WebSocketMessageData};
use crate::server::{Socket, WebSocketConnection};
use crate::services::greenhouse::Greenhouse;
use crate::services::session::Session;

fn greenhouses_update(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    let greenhouse_id = match message.data {
        WebSocketMessageData::RequestWithId { id } => id,
        _ => return Err(WebSocketErrorTemplate::BadRequest(None).into()),
    };

    let greenhouse = Greenhouse::find(greenhouse_id)?;
    let session = Session::find(connection.session_id.unwrap())?;
    let session_user_id = match session.user_id {
        Some(user_id) => user_id,
        None => return Err(WebSocketErrorTemplate::Unauthorized(None).into()),
    };

    if greenhouse.owner_id != session_user_id {
        return Err(WebSocketErrorTemplate::Forbidden(None).into());
    }

    let response = DispatchMessage {
        event: DispatchEvent::GreenhouseUpdate { id: greenhouse.id },
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

fn greenhouses_mine_update(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    let session = Session::find(connection.session_id.unwrap())?;

    let session_user_id = match session.user_id {
        Some(user_id) => user_id,
        None => return Err(WebSocketErrorTemplate::Unauthorized(None).into()),
    };

    let greenhouses = Greenhouse::find_all_by_owner_id(session_user_id)?;


    for greenhouse in greenhouses {
        let response = DispatchMessage {
            event: DispatchEvent::GreenhouseUpdate { id: greenhouse.id },
            new_subscribers: Some(vec![connection.id]),
        };

        Socket::send_message(
            message.id,
            response,
            connection.address.downgrade().recipient(),
            connection,
            context,
        )?;
    }

    Ok(())
}

pub fn subscribe(
    to: String,
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    match to.as_str() {
        "greenhouse" => greenhouses_update(message, connection, context)?,
        "greenhouses/mine" => greenhouses_mine_update(message, connection, context)?,
        _ => return Err(WebSocketErrorTemplate::InvalidRequestField(None).into()),
    }

    Ok(())
}
