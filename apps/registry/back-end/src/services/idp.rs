// src/services/idp.rs
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

/// Request DTO for OAuth token exchange.
#[derive(Serialize)]
pub struct WwwTokenRequest<'a> {
    pub grant_type: &'a str,
    pub code: &'a str,
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub redirect_uri: &'a str,
}

/// User details returned from IdP.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WwwUser {
    pub id: String,
    pub username: String,
    pub nickname: String,
    pub avatar: Option<String>,
}

/// Token response from IdP.
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WwwTokenResponse {
    pub access_token: String,
    #[warn(dead_code)]
    pub refresh_token: String,
    pub user: WwwUser,
}

/// Exchanges an authorization code for IdP tokens.
pub async fn exchange_code(
    http_client: &Client,
    config: &crate::config::AppConfig,
    code: &str,
) -> Result<WwwTokenResponse, Box<dyn std::error::Error>> {
    let payload = WwwTokenRequest {
        grant_type: "authorization_code",
        code,
        client_id: &config.idp_client_id,
        client_secret: &config.idp_client_secret,
        redirect_uri: &config.idp_redirect_uri,
    };

    let response = http_client
        .post(&config.idp_base_url)
        .json(&payload)
        .send()
        .await?;

    if response.status() != StatusCode::OK {
        // Fail fast on non-200 responses to avoid silent logic bugs
        let error_text = response.text().await?;
        return Err(format!("Auth failed. IdP returned: {}", error_text).into());
    }

    let result = response.json::<WwwTokenResponse>().await?;
    Ok(result)
}
