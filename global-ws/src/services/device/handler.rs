use actix_web_actors::ws::WebsocketContext;

use crate::error::{WebSocketError, WebSocketErrorTemplate};
use crate::messages::{DispatchEvent, DispatchMessage, Method, Opcode, WebSocketMessage, WebSocketMessageData};
use crate::server::{Socket, WebSocketConnection};
use crate::services::device::Device;
use crate::services::greenhouse::Greenhouse;
use crate::services::session::Session;

fn patch_device(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    let (device_id, greenhouse_id, new_name) = match message.data {
        WebSocketMessageData::RequestPatchDevice {
            id,
            greenhouse_id,
            name,
        } => (id, greenhouse_id, name),
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

    let current_device = Device::find(device_id)?;

    if current_device.greenhouse_id != greenhouse.id {
        return Err(WebSocketErrorTemplate::NotFound(None).into());
    }

    if current_device.name != new_name {
        if let Some(name) = &new_name {
            Device::check_name_length(name)?;
        }

        let updated_device = Device::update_name(current_device.id, new_name)?;

        let response = DispatchMessage {
            event: DispatchEvent::DeviceUpdate { id: updated_device.id },
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

    // Response to request
    let response = WebSocketMessage {
        id: message.id,
        connection_id: connection.id,
        opcode: Opcode::Response,
        data: WebSocketMessageData::Response {
            code: 200,
            message: "Successfully updated".to_string(),
        },
        ..Default::default()
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

pub fn handle(
    request: String,
    method: Method,
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    match method {
        Method::Patch => match request.as_str() {
            "device" => patch_device(message, connection, context)?,
            _ => return Err(WebSocketErrorTemplate::InvalidRequestField(None).into()),
        },
        _ => return Err(WebSocketErrorTemplate::MethodNotAllowed(None).into()),
    }

    Ok(())
}
