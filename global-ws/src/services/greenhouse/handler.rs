use actix_web_actors::ws::WebsocketContext;

use crate::error::{WebSocketError, WebSocketErrorTemplate};
use crate::messages::{DispatchEvent, DispatchMessage, Method, Opcode, WebSocketMessage, WebSocketMessageData};
use crate::server::{Socket, WebSocketConnection};
use crate::services::device_record::DeviceRecord;
use crate::services::greenhouse::{Greenhouse, NewGreenhouse};
use crate::services::session::Session;
use crate::services::user::User;

fn create_greenhouse(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    let (name, token) = match message.data {
        WebSocketMessageData::RequestPostGreenhouse { name, token } => (name, token),
        _ => return Err(WebSocketErrorTemplate::BadRequest(None).into()),
    };

    Greenhouse::check_name_length(&name)?;
    Greenhouse::check_token_length(&token)?;

    let session = Session::find(connection.session_id.unwrap())?;
    let Some(session_user_id)
        = session.user_id else { return Err(WebSocketErrorTemplate::Unauthorized(None).into()) };

    match Greenhouse::find_by_token(token.to_owned()) {
        Ok(_) => return Err(WebSocketErrorTemplate::GreenhouseTokenTaken(None).into()),
        Err(error) => if error.http_code != 404 { return Err(error) },
    };

    if Greenhouse::count_by_owner_id(session_user_id.to_owned())? >= 15 {
        return Err(WebSocketErrorTemplate::GreenhousesTooMany(None).into());
    }

    let greenhouse = NewGreenhouse { name, token, owner_id: session_user_id };
    let greenhouse = Greenhouse::create(greenhouse)?;

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

    let responses = vec![
        // Notify all owner sessions
        DispatchMessage {
            event: DispatchEvent::GreenhouseCreate {
                id: Some(greenhouse.id),
                owner_id: session_user_id,
            },
            new_subscribers: None,
        },
        // Notify all those who are subscribed to this user
        DispatchMessage {
            event: DispatchEvent::UserUpdate { id: session_user_id },
            new_subscribers: None,
        },
        // Notify all user sessions that are subscribed to themselves
        DispatchMessage {
            event: DispatchEvent::UserMeUpdate { id: session_user_id },
            new_subscribers: None,
        },
    ];

    for response in responses {
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

fn patch_greenhouse(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    let WebSocketMessageData::RequestPatchGreenhouse {
        id: greenhouse_id,
        name: new_name,
        token: new_token,
        maximum_average_humidity: new_maximum_average_humidity,
        minimum_average_temperature: new_minimum_average_temperature,
    } = message.data else { return Err(WebSocketErrorTemplate::BadRequest(None).into()) };

    let session = Session::find(connection.session_id.unwrap())?;
    let Some(session_user_id)
        = session.user_id else { return Err(WebSocketErrorTemplate::Unauthorized(None).into()) };
    let greenhouse
        = Greenhouse::find_by_id_and_owner_id(greenhouse_id, session_user_id)?;

    if new_token != greenhouse.token {
        match Greenhouse::find_by_token(new_token.to_owned()) {
            Ok(_) => return Err(WebSocketErrorTemplate::GreenhouseTokenTaken(None).into()),
            Err(error) => if error.http_code != 404 { return Err(error) },
        };
    }

    if new_name != greenhouse.name
        || new_token != greenhouse.token
        || new_maximum_average_humidity != greenhouse.maximum_average_humidity
        || new_minimum_average_temperature != greenhouse.minimum_average_temperature {
        Greenhouse::check_name_length(&new_name)?;
        Greenhouse::check_token_length(&new_token)?;
        DeviceRecord::check_data_size(&new_maximum_average_humidity.unwrap_or(0.0))?;
        DeviceRecord::check_data_size(&new_minimum_average_temperature.unwrap_or(0.0))?;

        let greenhouse = Greenhouse::update(
            greenhouse.id,
            new_name,
            new_token,
            new_maximum_average_humidity,
            new_minimum_average_temperature,
        )?;

        let response = DispatchMessage {
            event: DispatchEvent::GreenhouseUpdate { id: greenhouse.id },
            new_subscribers: None,
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
            message: "Successfully deleted".to_string(),
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

fn delete_greenhouse(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    let WebSocketMessageData::RequestDeleteGreenhouse {
        id: greenhouse_id, current_password
    } = message.data else { return Err(WebSocketErrorTemplate::BadRequest(None).into()) };

    User::check_password_length(&current_password)?;

    let session = Session::find(connection.session_id.unwrap())?;
    let Some(session_user_id)
        = session.user_id else { return Err(WebSocketErrorTemplate::Unauthorized(None).into()) };
    let user = User::find(session_user_id)?;

    if passwd::verify(current_password, user.password_hash).is_err() {
        return Err(WebSocketErrorTemplate::IncorrectPassword(None).into());
    }

    let greenhouse
        = Greenhouse::find_by_id_and_owner_id(greenhouse_id, session_user_id)?;

    Greenhouse::delete(greenhouse.id)?;

    // Response to request
    let response = WebSocketMessage {
        id: message.id,
        connection_id: connection.id,
        opcode: Opcode::Response,
        data: WebSocketMessageData::Response {
            code: 200,
            message: "Successfully deleted".to_string(),
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

    let responses = vec![
        // Notify all owner sessions
        DispatchMessage {
            event: DispatchEvent::GreenhouseDelete {
                id: Some(greenhouse.id),
                owner_id: session_user_id,
            },
            new_subscribers: None,
        },
        // Notify all those who are subscribed to this user
        DispatchMessage {
            event: DispatchEvent::UserUpdate { id: session_user_id },
            new_subscribers: None,
        },
        // Notify all user sessions that are subscribed to themselves
        DispatchMessage {
            event: DispatchEvent::UserMeUpdate { id: session_user_id },
            new_subscribers: None,
        },
    ];

    for response in responses {
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

pub fn handle(
    request: String,
    method: Method,
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    match method {
        Method::Post => match request.as_str() {
            "greenhouse" => create_greenhouse(message, connection, context)?,
            _ => return Err(WebSocketErrorTemplate::InvalidRequestField(None).into()),
        },
        Method::Patch => match request.as_str() {
            "greenhouse" => patch_greenhouse(message, connection, context)?,
            _ => return Err(WebSocketErrorTemplate::InvalidRequestField(None).into()),
        },
        Method::Delete => match request.as_str() {
            "greenhouse" => delete_greenhouse(message, connection, context)?,
            _ => return Err(WebSocketErrorTemplate::InvalidRequestField(None).into()),
        },
        _ => return Err(WebSocketErrorTemplate::MethodNotAllowed(None).into()),
    }

    Ok(())
}
