#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_files as fs;
use actix_web::{middleware, App, HttpServer};
use config::Config;

mod models;
mod handlers;
mod db;
mod config;
mod constants;
mod templates;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    println!("Starting service");

     let config_result = Config::init();
     println!("Environment variables: {:?}", config_result);
 

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(handlers::index)
            .service(handlers::get_data)
            .service(handlers::get_data_range)
            .service(handlers::write_data)
            // Serve static files
            .service(fs::Files::new("/static", "./static"))
    })
    .bind("0.0.0.0:9091")?
    .run()
    .await
}