use actix_web_actors::ws::WebsocketContext;

use crate::error::{WebSocketError, WebSocketErrorTemplate};
use crate::messages::{DispatchEvent, DispatchMessage, WebSocketMessage, WebSocketMessageData};
use crate::server::{Socket, WebSocketConnection};
use crate::services::device::Device;
use crate::services::greenhouse::Greenhouse;
use crate::services::session::Session;

fn device_records_update(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    let WebSocketMessageData::SubscribeToDeviceRecordsUpdate { device_id, greenhouse_id }
        = message.data else { return Err(WebSocketErrorTemplate::BadRequest(None).into()) };

    let session = Session::find(connection.session_id.unwrap())?;
    let Some(session_user_id)
        = session.user_id else { return Err(WebSocketErrorTemplate::Unauthorized(None).into()) };
    let greenhouse
        = Greenhouse::find_by_id_and_owner_id(greenhouse_id, session_user_id)?;
    let device = Device::find_by_id_and_greenhouse_id(device_id, greenhouse.id)?;

    let response = DispatchMessage {
        event: DispatchEvent::DeviceRecordsUpdate { device_id: device.id },
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

fn device_records_average_update(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    let WebSocketMessageData::SubscribeToDeviceRecordsAverageUpdate {
        device_id,
        greenhouse_id,
        range,
    } = message.data else { return Err(WebSocketErrorTemplate::BadRequest(None).into()) };

    let session = Session::find(connection.session_id.unwrap())?;
    let Some(session_user_id)
        = session.user_id else { return Err(WebSocketErrorTemplate::Unauthorized(None).into()) };
    let greenhouse
        = Greenhouse::find_by_id_and_owner_id(greenhouse_id, session_user_id)?;
    let device = Device::find_by_id_and_greenhouse_id(device_id, greenhouse.id)?;

    let response = DispatchMessage {
        event: DispatchEvent::DeviceRecordsAverageUpdate { device_id: device.id, range },
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
        "device_records" => device_records_update(message, connection, context)?,
        "device_records/average" => device_records_average_update(message, connection, context)?,
        _ => return Err(WebSocketErrorTemplate::InvalidRequestField(None).into()),
    }

    Ok(())
}
