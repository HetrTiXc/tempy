use influxdb::InfluxDbWriteable;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, InfluxDbWriteable)]
pub struct TimeSeriesValue {
    pub time: DateTime<Utc>,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeSerie {
    pub name: String,
    pub columns: Vec<String>,
    pub values: Vec<TimeSeriesValue>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InfluxDbReturnPayload {
    pub statement_id: i64,
    pub series: Vec<TimeSerie>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InfluxDbResult {
    pub results: Vec<InfluxDbReturnPayload>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Range {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}