use std::env;

use lazy_static::lazy_static;

lazy_static! {
    static ref EXTERNAL_DEVICES_API_URL: String = {
        env::var("EXTERNAL_DEVICES_API_URL").expect("EXTERNAL_DEVICES_API_URL not set")
    };
}

pub fn get_external_devices_api_url() -> String
{
    EXTERNAL_DEVICES_API_URL.clone()
}

pub fn init() {
    info!("Initialize Garthen Environment Variables");

    lazy_static::initialize(&EXTERNAL_DEVICES_API_URL);
}
