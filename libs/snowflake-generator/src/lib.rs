#[macro_use]
extern crate log;

use std::env;
use std::sync::Mutex;
use std::time::{Duration, UNIX_EPOCH};

use lazy_static::lazy_static;
use snowflake::SnowflakeIdGenerator;

lazy_static! {
    static ref SNOWFLAKE_ID_GENERATOR: Mutex<SnowflakeIdGenerator> = {
        let machine_id = env::var("SNOWFLAKE_MACHINE_ID").expect("SNOWFLAKE_MACHINE_ID not set")
            .parse::<i32>().expect("SNOWFLAKE_MACHINE_ID must be i32");
        let node_id = env::var("SNOWFLAKE_NODE_ID").expect("SNOWFLAKE_NODE_ID not set")
            .parse::<i32>().expect("SNOWFLAKE_NODE_ID must be i32");

        Mutex::new(
            SnowflakeIdGenerator::with_epoch(
                machine_id,
                node_id,
                UNIX_EPOCH + Duration::from_millis(1672531200000)),
        )
    };
}

pub fn generate() -> i64 {
    SNOWFLAKE_ID_GENERATOR.lock().unwrap().real_time_generate()
}

pub fn init() {
    info!("Initialize Snowflake Generator");

    lazy_static::initialize(&SNOWFLAKE_ID_GENERATOR);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_id() {
        env::set_var("SNOWFLAKE_MACHINE_ID", "0");
        env::set_var("SNOWFLAKE_NODE_ID", "0");

        generate();
    }
}
