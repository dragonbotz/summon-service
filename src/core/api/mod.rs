//! This module contains the service's api implementation
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
use actix_web::{get, web, HttpResponse, Responder};
use log::{error, info};
use reqwest;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello and welcome to Dragon Bot Z's summon service!")
}

#[get("/summon")]
async fn summon(http_client: web::Data<reqwest::Client>) -> impl Responder {
    let response = http_client.get("http://dbz-portal-service:58181/").send().await;

    if let Err(error) = response {
        error!("[/summon] An error occured: {}", error);
        return HttpResponse::InternalServerError().body(format!("{error}"));
    }

    HttpResponse::Ok().body("Ok 👌")
}
