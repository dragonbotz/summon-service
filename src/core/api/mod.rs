//! This module contains the service's api implementation
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
use actix_web::{get, web, HttpResponse, Responder};
use dbzlib_rs::model::portal::PortalContent;
use log::{debug, error};
use reqwest::Response;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello and welcome to Dragon Bot Z's summon service!")
}

#[get("/summon")]
async fn summon(http_client: web::Data<reqwest::Client>) -> impl Responder {
    let response = http_client
        .get("http://dbz-portal-service:8080/get-content/1")
        .send()
        .await;

    if let Err(error) = response {
        error!("[/summon] An error occured: {}", error);
        return HttpResponse::InternalServerError().body(format!("{error}"));
    }
    let response: Response = response.unwrap();

    let content = response.json::<PortalContent>().await;
    if let Err(error) = content {
        error!(
            "[/summon] An error occured while fetching the portal content: {}",
            error
        );
        return HttpResponse::InternalServerError().body(format!("{error}"));
    }
    let content: PortalContent = content.unwrap();

    debug!("{:?}", content.characters());

    HttpResponse::Ok().body("Ok 👌")
}
