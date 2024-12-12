use chrono::{DateTime, Utc};
use tokio::time::{sleep, Duration};
use reqwest::Error;
use serde::Serialize;

mod simulator;

#[derive(Serialize)]
struct Payload {
    time: DateTime<Utc>,
    value: f64,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let now = Utc::now();
    let start = now - chrono::Duration::days(150);

    let client = reqwest::Client::new();
    let url = "http://localhost:9091/writeTimeseriesValue";

    let temperatures = simulator::simulate_temperatures(300);

    // Optionally, print the temperatures or do something with them
    for (day, day_temps) in temperatures.iter().enumerate() {
        for (hour, &temp) in day_temps.iter().enumerate() {
            println!("Day {:03} Hour {:02}: {:.1}°C", day + 1, hour, temp);

            let body = Payload {
                time: start + chrono::Duration::hours(hour as i64) + chrono::Duration::days(day as i64),
                value: temp,
            };
            let response = client.post(url)
                .json(&body) // Use `.json` to serialize and send JSON data
                .send()
                .await?;
    
            if response.status().is_success() {
                println!("Sent: {}", body.time);
            } else {
                println!("Failed to send: {}", body.time);
            }
    
            sleep(tokio::time::Duration::from_millis(5)).await;
        }
    }

    Ok(())
}
