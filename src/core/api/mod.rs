//! This module contains the service's api implementation
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
use actix_web::{Responder, get, HttpResponse};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello and welcome to Dragon Bot Z's summon service!")
}
