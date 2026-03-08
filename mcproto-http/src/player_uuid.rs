/*
 *
 *  * Created: 2026-3-8 2:23:19
 *  * File: player_uuid.rs
 *  * The whole project follows MIT LICENSE.
 *  * Copyright (c) 2026 The Open Team. All rights reserved.
 *
 */
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
}
/// 通过用户名获取 Mojang UUID
pub async fn username_to_uuid(username: String) -> Result<Uuid, NetworkError> {
    let url = format!("https://api.mojang.com/users/profiles/minecraft/{}", username);

    // 发请求
    let response = reqwest::get(&url)
        .await
        .map_err(|e| NetworkError::RequestError(e))?;

    if response.status() == 404 {
        return Err(NetworkError::NotFound(username));
    }

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| NetworkError::RequestError(e))?;

    let id_str = json["id"]
        .as_str()
        .ok_or(NetworkError::NotFound("Id of the user".to_string()))?;
    let uuid_str = format!(
        "{}-{}-{}-{}-{}",
        &id_str[0..8],
        &id_str[8..12],
        &id_str[12..16],
        &id_str[16..20],
        &id_str[20..32]
    );

    Uuid::parse_str(&uuid_str).map_err(|e| NetworkError::InvalidData(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_username_to_uuid() {
        let uuid = username_to_uuid("Notch".to_string()).await.unwrap();
        assert_eq!(uuid.to_string(), "069a79f4-44e9-4726-a5be-fca90e38aaf5");
    }
}