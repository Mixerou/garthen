use actix_web::{get, HttpResponse, web};

use crate::error::ApiError;

#[get("/ping")]
pub async fn ping() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().body("pong"))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(ping);
}
