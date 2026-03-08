#[cfg(test)]
mod tests {
    use mcproto_http::player_uuid::username_to_uuid;
    use mcproto_http::auth::*;
    #[tokio::test]
    async fn test_username_to_uuid() {
        let uuid = username_to_uuid("Notch".to_string()).await.unwrap();
        assert_eq!(uuid.to_string(), "069a79f4-44e9-4726-a5be-fca90e38aaf5");
    }
    #[tokio::test]
    async fn test_login() {
        let data = device_code::get_device_response().await.unwrap();
        dbg!("Device getting successfully!\nDevice Code: {}\nUser Code: {}\nInterval: {}\nExpires in {}\nVerification Uri: {}", &data.device_code, &data.user_code, &data.interval, &data.expires_in, &data.verification_uri);
        let token = device_code::get_token(data).await.unwrap();
        dbg!("Token getting successfully!\nAccess Token: {}\nRefresh Token: {}\nExpires in: {}", &token.access_token, &token.refresh_token, &token.expires_in);
    }
}