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
    #[allow(dead_code)]
    pub tokens: WwwTokens,
    pub user: WwwUser,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WwwTokens {
    #[allow(dead_code)]
    pub access_token: String,
    #[allow(dead_code)]
    pub refresh_token: String,
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

    let token_url = format!("{}/oauth/token", config.idp_base_url);

    let response = http_client.post(&token_url).json(&payload).send().await?;

    let status = response.status();
    let body_text = response.text().await?;

    if status != StatusCode::OK {
        return Err(format!("Auth failed. IdP returned: {}", body_text).into());
    }

    let result: WwwTokenResponse = match serde_json::from_str(&body_text) {
        Ok(res) => res,
        Err(e) => {
            return Err(format!("JSON decode failed: {}", e).into());
        }
    };

    Ok(result)
}
