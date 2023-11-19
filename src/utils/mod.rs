//! Utilities
//!
//! This file contains the implementation of utilities functions
//!
//! Authors: Lahcène Belhadi
use dbzlib_rs::model::character::Character;
use dbzlib_rs::model::portal::PortalContent;
use dbzlib_rs::util::exception::{ExcResult, Exception};
use rand::seq::SliceRandom;

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
    match characters.choose(&mut rand::thread_rng()) {
        Some(character) => return Ok(character.clone()),
        None => {
            return Err(Exception::DrawCharacter(
                "The character Vector is empty".to_string(),
            ))
        }
    };
}
