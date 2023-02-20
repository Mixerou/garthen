use actix_web_actors::ws::WebsocketContext;

use crate::error::{WebSocketError, WebSocketErrorTemplate};
use crate::messages::{DispatchEvent, DispatchMessage, Method, Opcode, WebSocketMessage, WebSocketMessageData};
use crate::server::{Socket, WebSocketConnection};
use crate::services::greenhouse::{Greenhouse, NewGreenhouse};
use crate::services::session::Session;

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
    let session_user_id = match session.user_id {
        Some(user_id) => user_id,
        None => return Err(WebSocketErrorTemplate::Unauthorized(None).into()),
    };

    match Greenhouse::find_by_token(token.to_owned()) {
        Ok(_) => return Err(WebSocketErrorTemplate::GreenhouseTokenTaken(None).into()),
        Err(error) => if error.http_code != 404 { return Err(error) },
    };

    if Greenhouse::count_by_owner_id(session_user_id.to_owned())? >= 15 {
        return Err(WebSocketErrorTemplate::GreenhousesTooMany(None).into());
    }

    let greenhouse = NewGreenhouse {
        name,
        token,
        owner_id: session_user_id,
    };
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
        connection.address.downgrade().recipient(),
        connection,
        context,
    )?;

    // Notify all owner sessions
    let response = DispatchMessage {
        event: DispatchEvent::GreenhouseCreate {
            id: Some(greenhouse.id),
            owner_id: session_user_id,
        },
        new_subscribers: None,
    };

    Socket::send_message(
        message.id,
        response,
        connection.address.downgrade().recipient(),
        connection,
        context,
    )?;

    // Notify all those who are subscribed to this user
    let response = DispatchMessage {
        event: DispatchEvent::UserUpdate { id: session_user_id },
        new_subscribers: None,
    };

    Socket::send_message(
        message.id,
        response,
        connection.address.downgrade().recipient(),
        connection,
        context,
    )?;

    // Notify all user sessions that are subscribed to themselves
    let response = DispatchMessage {
        event: DispatchEvent::UserMeUpdate { id: session_user_id },
        new_subscribers: None,
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
        _ => return Err(WebSocketErrorTemplate::MethodNotAllowed(None).into()),
    }

    Ok(())
}
