#[macro_use]
extern crate log;

use std::env;

use futures::executor::block_on;
use lapin::{Connection, ConnectionProperties};
use lazy_static::lazy_static;

lazy_static! {
    static ref CONNECTION: Connection = {
        let amqp_url = env::var("AMQP_URL").expect("AMQP_URL not set");

        block_on(async move {
            Connection::connect(
                &amqp_url,
                ConnectionProperties::default(),
            ).await.expect("Failed to connect to the server")
        })
    };
}

pub fn get_connection() -> &'static Connection {
    &CONNECTION
}

pub fn init() {
    info!("Initialize AMQP");

    lazy_static::initialize(&CONNECTION);
}
