use serde::Deserialize;
use crate::NetworkError;
use reqwest::Client;
const CLIENT_ID: &str = "18a1a4c2-ccae-4306-9e55-e9500a1793d7";
#[derive(Deserialize)]
pub struct DeviceCodeData {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: i32,
    pub interval: i32,
}
#[derive(Deserialize)]
pub struct TokenData {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i32,
}
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: String, // 错误代码，
    pub error_description: Option<String>,
    pub error_codes: Option<Vec<i32>>,
    pub timestamp: Option<String>,
    pub trace_id: Option<String>,
    pub correlation_id: Option<String>,
}
pub async fn get_device_response() -> Result<DeviceCodeData, NetworkError> {
    // 获取设备码
    let url = "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode";
    let params = [
        ("client_id", CLIENT_ID),
        ("scope", "XboxLive.signin offline_access"),
    ];
    let client = Client::new();
    let response = client
        .post(url)
        .form(&params)
        .send()
        .await?;
    let data = response.json::<DeviceCodeData>().await?;
    Ok(data)
}
pub async fn get_token(data: DeviceCodeData) -> Result<TokenData, NetworkError> {
    let url = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";
    let params = [
        ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
        ("client_id", CLIENT_ID),
        ("device_code", &data.device_code),
    ];
    let client = Client::new();
    let mut interval = data.interval as u64;
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
        let response = client
            .post(url)
            .form(&params)
            .send()
            .await?;
        if response.status().is_success() {
            let token_data = response.json::<TokenData>().await?;
            break Ok(token_data);
        } else {
            let error = response.json::<ErrorResponse>().await?;
            match error.error.as_str() {
                "authorization_pending" => continue,
                "slow_down" => interval += 5,
                "expired_token" => {
                    println!("Token expired, please try again.");
                    break Err(NetworkError::TimeoutError);
                }
                _ => {
                    break Err(NetworkError::Other(format!("Error: {}\nDescription: {}", error.error, error.error_description.unwrap_or("None".to_string()))));
                }
            }
        }
    }
}