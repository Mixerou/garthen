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
    let (device_id, greenhouse_id) = match message.data {
        WebSocketMessageData::SubscribeToDeviceRecordsUpdate { device_id, greenhouse_id } => {
            (device_id, greenhouse_id)
        },
        _ => return Err(WebSocketErrorTemplate::BadRequest(None).into()),
    };
    let session = Session::find(connection.session_id.unwrap())?;
    let session_user_id = match session.user_id {
        Some(user_id) => user_id,
        None => return Err(WebSocketErrorTemplate::Unauthorized(None).into()),
    };
    let greenhouse = Greenhouse::find(greenhouse_id)?;

    if greenhouse.owner_id != session_user_id {
        return Err(WebSocketErrorTemplate::Forbidden(None).into());
    }

    let device = Device::find(device_id)?;

    if device.greenhouse_id != greenhouse.id {
        return Err(WebSocketErrorTemplate::NotFound(None).into());
    }

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
    let (device_id, greenhouse_id, range) = match message.data {
        WebSocketMessageData::SubscribeToDeviceRecordsAverageUpdate {
            device_id,
            greenhouse_id,
            range,
        } => {
            (device_id, greenhouse_id, range)
        },
        _ => return Err(WebSocketErrorTemplate::BadRequest(None).into()),
    };
    let session = Session::find(connection.session_id.unwrap())?;
    let session_user_id = match session.user_id {
        Some(user_id) => user_id,
        None => return Err(WebSocketErrorTemplate::Unauthorized(None).into()),
    };
    let greenhouse = Greenhouse::find(greenhouse_id)?;

    if greenhouse.owner_id != session_user_id {
        return Err(WebSocketErrorTemplate::Forbidden(None).into());
    }

    let device = Device::find(device_id)?;

    if device.greenhouse_id != greenhouse.id {
        return Err(WebSocketErrorTemplate::NotFound(None).into());
    }

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
