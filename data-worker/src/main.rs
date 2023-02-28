#[macro_use]
extern crate log;

use std::thread;
use std::time::Duration;

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    env_logger::init();

    info!("Starting worker");

    thread::spawn(|| {
        info!("Starting data requesting thread");

        loop {
            thread::sleep(Duration::from_secs(60));

            debug!("Want to request data");
        }
    }).join().expect("Couldn't join on the data requesting thread");
}
