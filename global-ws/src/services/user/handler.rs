use std::io::ErrorKind;

use actix_web_actors::ws::WebsocketContext;

use crate::error::{WebSocketError, WebSocketErrorKind, WebSocketErrorTemplate};
use crate::messages::{DispatchEvent, DispatchMessage, Method, Opcode, WebSocketMessage, WebSocketMessageData};
use crate::server::{Socket, WebSocketConnection};
use crate::services::session::Session;
use crate::services::user::{User, UserLocale, UserMe, UserPublic};
use crate::utils::dns;

fn patch_user(
    message: WebSocketMessage,
    connection: &mut WebSocketConnection,
    context: &mut WebsocketContext<WebSocketConnection>,
) -> Result<(), WebSocketError> {
    let (
        new_email,
        new_username,
        new_locale,
        new_theme,
        new_password,
        current_password,
    )
        = match message.data {
        WebSocketMessageData::RequestPatchUser {
            email,
            username,
            locale,
            theme,
            new_password,
            current_password,
        } => (email, username, locale, theme, new_password, current_password),
        _ => return Err(WebSocketErrorTemplate::BadRequest(None).into()),
    };

    let new_locale = match new_locale.parse::<UserLocale>() {
        Ok(locale) => locale,
        Err(_) => return Err(WebSocketErrorTemplate::BadRequest(None).into()),
    };

    let session = Session::find(connection.session_id.unwrap())?;
    let session_user_id = match session.user_id {
        Some(user_id) => user_id,
        None => return Err(WebSocketErrorTemplate::Unauthorized(None).into()),
    };

    let current_user = User::find(session_user_id.to_owned())?;

    let updated_user = match true {
        _ if current_user.email != new_email
            || current_user.username != new_username
            || new_password.is_some() => {
            let current_password = current_password.unwrap_or("".to_string());
            let new_password = new_password.unwrap_or(current_password.to_owned());

            User::check_email_length(&new_email)?;
            User::check_password_length(&current_password)?;
            User::check_password_length(&new_password)?;
            User::check_username_length(&new_username)?;

            if passwd::verify(
                current_password.to_owned(),
                current_user.password_hash.to_owned(),
            ).is_err() {
                return Err(WebSocketErrorTemplate::IncorrectPassword(None).into());
            }

            let new_email = match new_email {
                email if email == current_user.email => current_user.email.to_owned(),
                email => {
                    let email_domain: Vec<&str> = email.split('@').collect();

                    if email_domain.len() != 2 {
                        return Err(WebSocketErrorTemplate::EmailInvalid(None).into());
                    }

                    let domain_mx_records
                        = match dns::get_mx_records(email_domain[1]) {
                        Ok(records) => records,
                        Err(error) => {
                            if let WebSocketErrorKind::StdError(std_error) = &error.kind {
                                if std_error.kind() == ErrorKind::InvalidData {
                                    return Err(WebSocketErrorTemplate::EmailInvalid(None).into());
                                }
                            }

                            return Err(error);
                        }
                    };

                    if domain_mx_records.is_empty()
                        || User::find_by_email(email.to_owned()).is_ok() {
                        return Err(WebSocketErrorTemplate::EmailInvalid(None).into());
                    }

                    email
                },
            };

            let new_password_hash = match new_password {
                password if password == current_password =>
                    current_user.password_hash.to_owned(),
                password => passwd::hash(password)?,
            };

            let new_username = match new_username {
                username if username == current_user.username => username,
                username => {
                    if User::find_by_username(username.to_owned()).is_ok() {
                        return Err(WebSocketErrorTemplate::UsernameInvalidOrTaken(None).into());
                    }

                    username
                }
            };

            User::hard_update(
                session_user_id,
                new_email,
                new_password_hash,
                new_username,
                new_locale,
                new_theme,
            )?
        },
        _ if new_locale != current_user.locale || new_theme != current_user.theme => {
            User::soft_update(session_user_id, new_locale, new_theme)?
        },
        _ => current_user.clone(),
    };

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

    if updated_user != current_user {
        let current_user = UserMe::try_from(current_user)?;
        let updated_user = UserMe::try_from(updated_user)?;

        if current_user != updated_user {
            // Notify all user sessions that are subscribed to themselves
            let response = DispatchMessage {
                event: DispatchEvent::UserMeUpdate { id: session_user_id },
                new_subscribers: None,
            };

            Socket::send_message(
                message.id,
                response,
                connection.socket.downgrade().recipient(),
                connection,
                context,
            )?;

            if UserPublic::from(updated_user) != UserPublic::from(current_user) {
                // Notify all those who are subscribed to this user
                let response = DispatchMessage {
                    event: DispatchEvent::UserUpdate { id: session_user_id },
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
        }
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
        Method::Patch => match request.as_str() {
            "user/me" => patch_user(message, connection, context)?,
            _ => return Err(WebSocketErrorTemplate::InvalidRequestField(None).into()),
        },
        _ => return Err(WebSocketErrorTemplate::MethodNotAllowed(None).into()),
    }

    Ok(())
}
