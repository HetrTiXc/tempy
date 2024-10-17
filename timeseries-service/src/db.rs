use influxdb::Client;

use crate::config::{INFLUXDB_URL, INFLUXDB_DATABASE, INFLUXDB_TOKEN};

pub fn get_influxdb_client() -> Client {
    Client::new(INFLUXDB_URL, INFLUXDB_DATABASE).with_token(INFLUXDB_TOKEN)
}