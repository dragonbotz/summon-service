//! This module contains the service's api implementation
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
use crate::utils;
use actix_web::{get, web, HttpResponse, Responder};
use dbzlib_rs::model::{character::Character, portal::PortalContent};
use log::debug;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello and welcome to Dragon Bot Z's summon service!")
}

/// Retrieves current portal data and then draws a random character from the
/// portal content. If an error occured during the process, returns an empty
/// JSON
#[get("/summon")]
async fn summon(http_client: web::Data<reqwest::Client>) -> impl Responder {
    // fetches current portal content
    let portal_content_result = utils::get_current_portal_content(&http_client).await;

    if let Err(error) = portal_content_result {
        debug!("{}", error);
        return HttpResponse::Ok().json({});
    }
    let portal_content: PortalContent = portal_content_result.unwrap();

    // resolve characters
    let characters_result =
        utils::get_characters_from_portal_content(&http_client, &portal_content).await;

    if let Err(error) = characters_result {
        debug!("{}", error);
        return HttpResponse::Ok().json({});
    }
    let characters: Vec<Character> = characters_result.unwrap();

    // draw a character from the vector
    let character = utils::draw_character_from_vec(&characters);

    if let Err(error) = character {
        debug!("{}", error);
        return HttpResponse::Ok().json({});
    }

    HttpResponse::Ok().json(character.unwrap())
}
