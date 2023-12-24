//! Utilities
//!
//! This file contains the implementation of utilities functions
//!
//! Authors: Lahcène Belhadi
use dbzlib_rs::model::character::Character;
use dbzlib_rs::model::portal::PortalContent;
use dbzlib_rs::util::exception::{ExcResult, Exception};
use rand::prelude::*;
use std::collections::HashMap;

/// Retrieves current portal content
///
/// # Arguments
/// * http_client: `reqwest::Client` - the http client to use
pub async fn get_current_portal_content(http_client: &reqwest::Client) -> ExcResult<PortalContent> {
    let response = match http_client
        .get("http://dbz-portal-service:8080/get-content/1")
        .send()
        .await
    {
        Ok(response) => response,
        Err(error) => return Err(Exception::RetrievePortalContent(format!("{error}"))),
    };

    let portal_content = response.json::<PortalContent>().await;
    if let Err(error) = portal_content {
        return Err(Exception::RetrievePortalContent(format!("{error}")));
    }

    Ok(portal_content.unwrap())
}

/// Retrieves characters informations from `PortalContent`
///
/// # Arguments
/// * http_client: `reqwest::Client` - the http client to use
/// * portal_content: `PortalContent` - the portal content to retrieve characters from
pub async fn get_characters_from_portal_content(
    http_client: &reqwest::Client,
    portal_content: &PortalContent,
) -> ExcResult<Vec<Character>> {
    let response = match http_client
        .get("http://dbz-character-service:8080/get-many")
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .json(&portal_content.characters())
        .send()
        .await
    {
        Ok(response) => response,
        Err(error) => return Err(Exception::RetrieveMultipleCharacters(error.to_string())),
    };

    let characters = response.json::<Vec<Character>>().await;
    if let Err(error) = characters {
        return Err(Exception::RetrieveMultipleCharacters(error.to_string()));
    }

    Ok(characters.unwrap())
}

/// Draws a random character from a `Vec<Character>`
///
/// # Arguments
/// * characters: `Vec<Character>` - the vector containing the characters
pub fn draw_character_from_vec(characters: &Vec<Character>) -> ExcResult<Character> {
    let characters_sorted: HashMap<i16, Vec<Character>> = sort_characters_by_rarity(characters);

    // droprates
    let droprate_uncommon = 0.4;
    let droprate_super = 0.2;
    let droprate_extreme = 0.1;
    let droprate_extreme_origin = 0.01;

    // generate random number
    let random_number: f32 = rand::thread_rng().gen();

    let mut draw_from: &Vec<Character> = &vec![];
    if random_number <= droprate_extreme_origin {
        draw_from = &characters_sorted[&0]; 
    } else if random_number <= droprate_extreme {
        draw_from = &characters_sorted[&1];
    } else if random_number <= droprate_super {
        draw_from = &characters_sorted[&2];
    } else if random_number <= droprate_uncommon {
        draw_from = &characters_sorted[&3];
    } else {
        draw_from = &characters_sorted[&4];
    }

    match draw_from.choose(&mut rand::thread_rng()) {
        Some(character) => return Ok(character.clone()),
        None => {
            return Err(Exception::DrawCharacter(
                "The character Vector is empty".to_string(),
            ))
        }
    };
}

/// Sort characters by rarity
///
/// # Arguments
/// * characters: `Vec<Character> - The character vector
///
/// # Returns
/// A Map containing the characters sorted by rarity
fn sort_characters_by_rarity(characters: &Vec<Character>) -> HashMap<i16, Vec<Character>> {
    let mut map: HashMap<i16, Vec<Character>> = HashMap::new();

    // init keys
    for i in 0..6 {
        map.insert(i, vec![]);
    }

    // insert characters
    for character in characters {
        map.get_mut(&character.rarity())
            .map(|value| value.push(character.clone()));
    }

    map
}
