use std::env;
use std::io;
use std::sync;

#[derive(Debug, Clone)]
pub struct Config {
    pub influx_url: String,
    pub influx_org: String,
    pub influx_bucket: String,
    pub influx_token: String,
    pub server_host: String,
    pub server_port: u16,
    pub log_level: String,
}

// Define a single static instance
static INSTANCE: sync::OnceLock<Config> = sync::OnceLock::new();

impl Config {
    pub fn new() -> Result<Config, String> {
        println!("Initializing config...");

        Ok(Config {
            influx_url: env::var("INFLUX_URL")
                .unwrap_or_else(|_| String::from("http://localhost:8086")),
            influx_org: env::var("INFLUX_ORG")
                .map_err(|_| "INFLUX_ORG not set")?,
            influx_bucket: env::var("INFLUX_BUCKET")
                .map_err(|_| "INFLUX_BUCKET not set")?,
            influx_token: env::var("INFLUX_TOKEN")
                .map_err(|_| "INFLUX_TOKEN not set")?,
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| String::from("127.0.0.1")),
            server_port: env::var("SERVER_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            log_level: env::var("RUST_LOG")
                .unwrap_or_else(|_| String::from("info")),
        })
    }

    // Singleton pattern to store the config
    pub fn global() -> &'static Config {
        INSTANCE.get().expect("Config not initialized")
    }

    // Initialize the global config
    pub fn init() -> Result<&'static Config, io::Error> {
        println!("Starting config initialization...");
        let config = Config::new().map_err(|e| {
            println!("Config initialization error: {}", e);
            io::Error::new(io::ErrorKind::Other, e)
        })?;
        println!("Config created successfully");
        let result = INSTANCE.get_or_init(|| config);
        println!("Config initialized globally");
        Ok(result)
    }
}