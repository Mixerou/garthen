#[macro_use]
extern crate log;
extern crate snowflake_generator as snowflake;

use dotenv::dotenv;

use crate::services::device_record;

mod error;
mod garthen;
mod services;

fn main() {
    dotenv().ok();
    env_logger::init();

    db::init();
    snowflake::init();
    garthen::init();

    info!("Starting worker");

    let data_requesting_thread
        = device_record::start_data_requesting_with_interval();

    data_requesting_thread.join()
        .expect("Couldn't join on the data requesting thread")
        .expect("Failed to successfully finish data requesting thread");
}
