use std::time::{Duration, SystemTime, UNIX_EPOCH};

use actix_broker::{Broker, SystemBroker};
use actix_web_actors::ws::WebsocketContext;

use crate::error::{WebSocketError, WebSocketErrorTemplate};
use crate::messages::{AmqpPayload, AmqpPublisherMessage, DispatchEvent, DispatchMessage, Method, Opcode, WebSocketMessage, WebSocketMessageData};
use crate::server::{Socket, WebSocketConnection};
use crate::services::device::{Device, DeviceKind, DeviceStatus};
use crate::services::device_record::{DeviceRecord, NewDeviceRecord};
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

fn patch_device_state(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    let (device_id, greenhouse_id, state) = match message.data {
        WebSocketMessageData::RequestPatchDeviceState {
            id,
            greenhouse_id,
            state,
        } => (id, greenhouse_id, state),
        _ => return Err(WebSocketErrorTemplate::BadRequest(None).into()),
    };

    if state > 1 { return Err(WebSocketErrorTemplate::InvalidDeviceState(None).into()); }

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
    if device.kind != DeviceKind::HumidificationController
        && device.kind != DeviceKind::IrrigationController
        && device.kind != DeviceKind::WindowsController {
        return Err(WebSocketErrorTemplate::DeviceIsNotController(None).into());
    }

    Broker::<SystemBroker>::issue_async(AmqpPublisherMessage {
        exchange: Some("device"),
        routing_key: Some("device.controller.state.change"),
        payload: AmqpPayload::ChangeControllerState {
            device_id,
            state,
        },
    });

    // Response to request
    let response = WebSocketMessage {
        id: message.id,
        connection_id: connection.id,
        opcode: Opcode::Response,
        data: WebSocketMessageData::Response {
            code: 200,
            message: "Successfully requested".to_string(),
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

fn post_device_custom_data(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    let (device_id, greenhouse_id, data, time) = match message.data {
        WebSocketMessageData::RequestPostDeviceCustomData {
            id,
            greenhouse_id,
            data,
            time,
        } => (id, greenhouse_id, data, time),
        _ => return Err(WebSocketErrorTemplate::BadRequest(None).into()),
    };

    DeviceRecord::check_data_size(&data)?;

    let time = Duration::from_secs(time);
    let three_month_ago = (SystemTime::now() - Duration::from_secs(2629743 * 3))
        .duration_since(UNIX_EPOCH).unwrap();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    match time {
        time if time < three_month_ago => return Err(
            WebSocketErrorTemplate::TooLongAgo(None).into()
        ),
        time if time > now => return Err(
            WebSocketErrorTemplate::FutureTime(None).into()
        ),
        _ => {},
    }

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
    if device.kind != DeviceKind::HumiditySensor
        && device.kind != DeviceKind::SoilMoistureSensor
        && device.kind != DeviceKind::TemperatureSensor {
        return Err(WebSocketErrorTemplate::DeviceIsNotSensor(None).into());
    }

    let record = NewDeviceRecord {
        device_id: device.id,
        data,
    };
    let record = DeviceRecord::create_with_custom_time(
        record,
        UNIX_EPOCH + time)?;

    // Response to request
    let response = WebSocketMessage {
        id: message.id,
        connection_id: connection.id,
        opcode: Opcode::Response,
        data: WebSocketMessageData::Response {
            code: 200,
            message: "Successfully created".to_string(),
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

    if record.id == DeviceRecord::find_latest_by_device_id(device.id)?.id {
        // Notify all those who are subscribed to this device
        let response = DispatchMessage {
            event: DispatchEvent::DeviceUpdate { id: device.id },
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

fn post_device_request_data(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    let (device_id, greenhouse_id) = match message.data {
        WebSocketMessageData::RequestPostDeviceRequestData {
            id,
            greenhouse_id,
        } => (id, greenhouse_id),
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

    if let Some(device_id) = device_id {
        let device = Device::find(device_id)?;

        if device.greenhouse_id != greenhouse.id {
            return Err(WebSocketErrorTemplate::NotFound(None).into());
        }
    }

    Broker::<SystemBroker>::issue_async(AmqpPublisherMessage {
        exchange: Some("data"),
        routing_key: Some("data.request"),
        payload: AmqpPayload::RequestData {
            device_id,
            greenhouse_id: Some(greenhouse_id),
        },
    });

    // Response to request
    let response = WebSocketMessage {
        id: message.id,
        connection_id: connection.id,
        opcode: Opcode::Response,
        data: WebSocketMessageData::Response {
            code: 200,
            message: "Successfully requested".to_string(),
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

fn post_device_status(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
    new_status: DeviceStatus,
) -> Result<(), WebSocketError> {
    let (device_id, greenhouse_id) = match message.data {
        WebSocketMessageData::RequestPostDeviceDisable {
            id,
            greenhouse_id,
        } => (id, greenhouse_id),
        WebSocketMessageData::RequestPostDeviceEnable {
            id,
            greenhouse_id,
        } => (id, greenhouse_id),
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

    if device.status != new_status {
        Device::update_status(device.id, new_status)?;

        // Notify all those who are subscribed to this device
        let response = DispatchMessage {
            event: DispatchEvent::DeviceUpdate { id: device.id },
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
            message: match new_status {
                DeviceStatus::Online => "Successfully enabled",
                DeviceStatus::Disabled => "Successfully disabled",
                _ => "Successfully",
            }.to_string(),
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
            "device/state" => patch_device_state(message, connection, context)?,
            _ => return Err(WebSocketErrorTemplate::InvalidRequestField(None).into()),
        },
        Method::Post => match request.as_str() {
            "device/custom-data" => post_device_custom_data(message, connection, context)?,
            "device/request-data" => post_device_request_data(message, connection, context)?,
            "device/disable" => post_device_status(
                message,
                connection,
                context,
                DeviceStatus::Disabled,
            )?,
            "device/enable" => post_device_status(
                message,
                connection,
                context,
                DeviceStatus::Online,
            )?,
            _ => return Err(WebSocketErrorTemplate::InvalidRequestField(None).into()),
        },
        _ => return Err(WebSocketErrorTemplate::MethodNotAllowed(None).into()),
    }

    Ok(())
}
