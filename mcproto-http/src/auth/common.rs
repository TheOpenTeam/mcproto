use log::error;
use reqwest::Client;
use serde_json::json;

use crate::{NetworkError, auth::device_code::TokenData};
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct XboxAuthResponse { // 草 不知道为啥这就成大驼峰了
    #[serde(rename = "IssueInstant")]
    pub issue_instant: String,
    #[serde(rename = "NotAfter")]
    pub not_after: String,
    #[serde(rename = "Token")]
    pub token: String,
    #[serde(rename = "DisplayClaims")]
    pub display_claims: DisplayClaims
}
#[derive(Serialize, Deserialize)]
pub struct DisplayClaims {
    pub xui: Vec<Xui>
}
#[derive(Serialize, Deserialize)]
pub struct Xui {
    pub uhs: String
}

#[derive(Serialize, Deserialize)]
pub struct MinecraftTokenResponse {
    pub username: String,
    pub roles: Vec<String>, // empty
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,

}
pub async fn xbox_live_auth(client: &Client, data: TokenData) -> Result<XboxAuthResponse, NetworkError>{
    let url = "https://user.auth.xboxlive.com/user/authenticate";
    let body = json!({
        "Properties": {
            "AuthMethod": "RPS",
            "SiteName": "user.auth.xboxlive.com",
            "RpsTicket": format!("d={}", data.access_token),
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT"

    });
    let response = 
    client.post(url)
    .json(&body)
    .send()
    .await?;
    let stat = response.status();
    if stat.is_success() {
        let result: XboxAuthResponse = response.json().await?;
        return Ok(result);
    } else {
        error!("Error authenticating in Xbox Live. Error code: {}", stat);
        return Err(NetworkError::Other(stat.to_string()));
    }

    

}
pub async fn xsts_auth(client: &Client, data: XboxAuthResponse) -> Result<XboxAuthResponse, NetworkError>{
    let url = "https://xsts.auth.xboxlive.com/xsts/authorize";
    let body = json!({
    "Properties": {
        "SandboxId": "RETAIL",
        "UserTokens": [format!("{}", &data.token)]
    },
    "RelyingParty": "rp://api.minecraftservices.com/",
    "TokenType": "JWT"
}
    );
    let response = 
    client
    .post(url)
    .json(&body)
    .send()
    .await?;
    let stat = response.status();
    if stat.is_success() {
        let result: XboxAuthResponse = response.json().await?;
        return Ok(result);
    } else {
        error!("Error authenticating in XSTS Auth. Error code: {}", stat);
        return Err(NetworkError::Other(stat.to_string()));
    }

}

pub async fn get_mc_access_token(client: &Client, data: XboxAuthResponse) -> Result<MinecraftTokenResponse, NetworkError>{
    let url = "https://api.minecraftservices.com/authentication/login_with_xbox";
    let uhs = &data.display_claims.xui[0].uhs;
    let body = json!(
        {
            "identityToken": format!("XBL3.0 x={};{}", uhs, &data.token)
        }

    );
    let response = 
    client
    .post(url)
    .json(&body)
    .send()
    .await?;
    let stat = response.status();
    if stat.is_success() {
        let result: MinecraftTokenResponse = response.json().await?;
        return Ok(result);
    } else {
        error!("Error authenticating in Minecraft Auth. Error code: {}", stat);
        return Err(NetworkError::Other(stat.to_string()));
    }

}
