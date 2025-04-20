use actix_web::{HttpResponse, post, web};
use chrono::Utc;
use askama::Template;

use crate::templates::{IndexTemplate, DataTemplate};

use crate::{constants::APPLICATION_JSON, models,};

extern crate influxdb;

use models::{TimeSerie, TimeSeriesValue, InfluxDbResult, InfluxDbReturnPayload, Range};
use crate::db::get_influxdb_client;
use influxdb::{Client, Error, ReadQuery, Timestamp, InfluxDbWriteable};

#[get("/")]
async fn index() -> HttpResponse {
    let index_template = IndexTemplate {};

    HttpResponse::Ok()
        .content_type("text/html")
        .body(index_template.render().unwrap())
}

#[get("/getTimeseriesValue")]
pub async fn get_data() -> HttpResponse {
    let client = get_influxdb_client();

    let now = Utc::now();
    let read_query = ReadQuery::new(format!("SELECT * FROM watertemp WHERE time <= '{}' order by time desc limit 1", now.to_rfc3339()));
    match client.query(read_query).await {
        Ok(read_result) => {
            // Assuming InfluxDB returns a JSON-like result here
            // let time_series: Vec<TimeSeriesValue> = read_result.deserialize().expect("Deserialize error");

            println!("{}", read_result);
            
            let json: InfluxDbResult = serde_json::from_str(&read_result).expect("Deserialization Error");

            HttpResponse::Ok()
                .content_type(APPLICATION_JSON)
                .json(json)
        },
        Err(e) => {
            eprintln!("Error querying timeseries data: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

#[post("/getTimeseriesRange")]
pub async fn get_data_range(range: web::Json<Range>) -> HttpResponse {
    println!("Serialized: {}", serde_json::to_string(&range).unwrap());

    let json_item = serde_json::to_string(&range).unwrap();
    let time_range: Range = serde_json::from_str(&json_item).unwrap();

    let client = get_influxdb_client();

    let read_query_string = format!("SELECT * FROM watertemp WHERE time >= '{}'
  AND time <= '{}' order by time desc;", time_range.start.format("%Y-%m-%dT%H:%M:%S%.3fZ"), time_range.end.format("%Y-%m-%dT%H:%M:%S%.3fZ"));
    let read_query = ReadQuery::new(read_query_string);
    match client.query(read_query).await {
        Ok(read_result) => {
            println!("Serialized: {}", serde_json::to_string(&read_result).unwrap());
            let json: InfluxDbResult = serde_json::from_str(&read_result).expect("Deserialization Error");

            HttpResponse::Ok()
                .content_type(APPLICATION_JSON)
                .json(json)
        },
        Err(e) => {
            eprintln!("Error querying timeseries data: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

#[post("/writeTimeseriesValue")]
pub async fn write_data(measurement: web::Json<TimeSeriesValue>) -> HttpResponse {
    println!("Serialized: {}", serde_json::to_string(&measurement).unwrap());

    let json_item = serde_json::to_string(&measurement).unwrap();
    let item: TimeSeriesValue = serde_json::from_str(&json_item).unwrap();

    let now = Utc::now();
    let client = get_influxdb_client();
    let write_query = TimeSeriesValue {
        time: item.time,
        value: item.value,
    }
    .into_query("watertemp");
    
    match client.query(write_query).await {
        Ok(_) => HttpResponse::Ok().body("Write Successful"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }

    // HttpResponse::Created()
    // .content_type(APPLICATION_JSON)
    // .json(measurement)
}

// pub async fn write_timeseries_value(item: web::Json<TimeSeriesValue>) -> impl Responder {
//     let client = get_influxdb_client();
//     let write_query = influxdb::Timestamp::Nanoseconds(item.timestamp as u128 * 1_000_000)
//         .into_query("weather")
//         .add_field("temperature", item.value);
//     let write_result = client.query(&write_query).await;

//     match client.query(&point.into_query(crate::config::INFLUXDB_DATABASE)).await {
//         Ok(_) => HttpResponse::Ok().body("Write Successful"),
//         Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
//     }
// }