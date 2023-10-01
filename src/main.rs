//! Dragon Bot Z's summon service
//!
//! This is the entry point of the summon service
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
pub mod core;

use actix_web::{web, App, HttpServer};
use log::{error, info};
use reqwest;

use crate::core::api;

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let http_client = reqwest::Client::new();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(http_client.clone()))
            .service(api::root)
            .service(api::summon)
    })
    .bind(("0.0.0.0", 8083));

    if let Err(error) = server {
        error!("An error occured while setting up web server: {error}");
        std::process::exit(1);
    }
    let server = server.unwrap();

    info!("Starting web server: http://0.0.0.0:8083");
    if let Err(error) = server.run().await {
        error!("An error occured while starting up web server: {error}");
        std::process::exit(1);
    }
}
