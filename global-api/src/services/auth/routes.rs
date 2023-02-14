use actix_web::{HttpResponse, post, web};

use crate::error::ApiError;
use crate::services::auth::{Auth, LoginRequest, RegistrationRequest};
use crate::services::session::Session;

#[post("/auth/register")]
pub async fn register(
    session: web::ReqData<Session>,
    credentials: web::Json<RegistrationRequest>,
) -> Result<HttpResponse, ApiError> {
    // TODO: Email confirmation
    Auth::register(credentials.into_inner(), session.id)?;

    Ok(HttpResponse::NoContent().finish())
}

#[post("/auth/login")]
pub async fn login(
    session: web::ReqData<Session>,
    credentials: web::Json<LoginRequest>,
) -> Result<HttpResponse, ApiError> {
    Auth::login(credentials.into_inner(), session.id)?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
}
