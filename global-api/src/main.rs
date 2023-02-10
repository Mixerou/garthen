#[macro_use]
extern crate log;

use std::env;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::{NormalizePath, TrailingSlash};
use dotenv::dotenv;

mod error;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let ip = env::var("GLOBAL_API_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("GLOBAL_API_PORT").unwrap_or_else(|_| "5000".to_string());
    let path = env::var("GLOBAL_API_PATH").unwrap_or_else(|_| "".to_string());

    info!("Starting server on {ip} with port {port}");

    HttpServer::new(move || {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .service(
                web::scope(path.as_str())
                    .configure(services::system::init_routes)
            )
    })
        .bind(format!("{ip}:{port}"))?
        .run()
        .await
}
