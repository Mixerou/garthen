#[macro_use]
extern crate log;
extern crate snowflake_generator as snowflake;

use dotenv::dotenv;

use crate::services::device_record;

mod amqp_client;
mod error;
mod garthen;
mod services;

fn main() {
    dotenv().ok();
    env_logger::init();

    db::init();
    amqp::init();
    amqp_client::init();
    snowflake::init();
    garthen::init();

    info!("Starting worker");

    let data_requesting_thread
        = device_record::start_data_requesting_with_interval();
    let data_requester_consumer_thread
        = device_record::start_data_request_consumer();

    data_requesting_thread.join()
        .expect("Couldn't join on the data requesting thread")
        .expect("Failed to successfully finish data requesting thread");
    data_requester_consumer_thread.join()
        .expect("Couldn't join on the data-requester consumer thread")
        .expect("Failed to successfully finish data-requester consumer thread");
}
