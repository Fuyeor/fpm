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
    pub tokens: WwwTokens,
    pub user: WwwUser,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WwwTokens {
    pub access_token: String,
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

    /*
        println!(
            "\n[IdP Debug] 🚀 发起 OAuth 令牌交换...\n[IdP Debug] 🔗 请求 URL: {}\n[IdP Debug] 📝 回调地址: {}\n",
            token_url,
            config.idp_redirect_uri
        );
    */

    let response = http_client.post(&token_url).json(&payload).send().await?;

    let status = response.status();
    let body_text = response.text().await?;

    if status != StatusCode::OK {
        /*
        println!(
            "\n[IdP Debug] ❌ 请求失败! 状态码: {}\n[IdP Debug] 🔗 失败的 URL: {}\n[IdP Debug] 📥 主站原始报错: {}\n",
            status,
            token_url,
            body_text
        );
        */
        return Err(format!("Auth failed. IdP returned: {}", body_text).into());
    }

    let result: WwwTokenResponse = match serde_json::from_str(&body_text) {
        Ok(res) => res,
        Err(e) => {
            /*
            println!(
                "\n[IdP Debug] ❌ JSON 解析失败!\n[IdP Debug] 🔍 错误原因: {}\n[IdP Debug] 📥 主站返回的原始 JSON 数据: {}\n",
                e,
                body_text
            );
            */
            return Err(format!("JSON decode failed: {}", e).into());
        }
    };

    Ok(result)
}
