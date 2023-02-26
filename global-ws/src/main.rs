#[macro_use]
extern crate log;
extern crate snowflake_generator as snowflake;

use std::env;

use actix::{Actor, Addr};
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::web::{Data, get, Payload, Query, scope};
use actix_web_actors::ws;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use crate::server::{Encoding, Socket, WebSocketConnection};

mod error;
mod messages;
mod server;
mod services;
mod utils;

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryParams {
    encoding: Encoding,
}

async fn index(
    request: HttpRequest,
    stream: Payload,
    params: Query<QueryParams>,
    socket: Data<Addr<Socket>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WebSocketConnection::new(params.encoding, socket.into_inner()),
        &request,
        stream,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    db::init();
    snowflake::init();

    let data = Data::new(Socket::default().start());

    let ip = env::var("GLOBAL_WS_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("GLOBAL_WS_PORT").unwrap_or_else(|_| "9000".to_string());
    let path = env::var("GLOBAL_WS_PATH").unwrap_or_else(|_| "".to_string());

    info!("Starting server on {ip} with port {port}");

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(
                scope(
                    path.as_str())
                    .wrap(NormalizePath::new(TrailingSlash::Always))
                    .route("/", get().to(index))
            )
    })
        .bind(format!("{ip}:{port}"))?
        .run()
        .await
}
