//! Utilities
//!
//! This file contains the implementation of utilities functions
//!
//! Authors: Lahcène Belhadi
use dbzlib_rs::model::portal::PortalContent;
use dbzlib_rs::util::exception::{ExcResult, Exception};

/// Retrives current portal content
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
        return Err(Exception::RetrievePortalContent(format!("{error}")))
    }

    Ok(portal_content.unwrap())
}
