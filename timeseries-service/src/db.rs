use influxdb::Client;
use crate::config::Config;

//use crate::config::{INFLUXDB_URL, INFLUXDB_DATABASE, INFLUXDB_TOKEN};

pub fn get_influxdb_client() -> Client {
    let config = Config::global();
    println!("InfluxDB URL: {}", &config.influx_url);
    println!("InfluxDB Database: {}", &config.influx_org);
    println!("InfluxDB Token: {}", &config.influx_token);
    Client::new(&config.influx_url, &config.influx_org).with_token(&config.influx_token)
}