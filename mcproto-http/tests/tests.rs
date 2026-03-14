#[cfg(test)]
mod tests {
    use mcproto_http::player_uuid::username_to_uuid;
    use mcproto_http::auth::*;
    use reqwest::Client;
    #[tokio::test]
    async fn test_username_to_uuid() {
        let uuid = username_to_uuid("Notch".to_string()).await.unwrap();
        assert_eq!(uuid.to_string(), "069a79f4-44e9-4726-a5be-fca90e38aaf5");
    }
    #[tokio::test]
    async fn test_login() {
        let client = Client::new();
        let data = device_code::get_device_response(&client).await.unwrap();
        dbg!("Device getting successfully!\nDevice Code: {}\nUser Code: {}\nInterval: {}\nExpires in {}\nVerification Uri: {}", &data.device_code, &data.user_code, &data.interval, &data.expires_in, &data.verification_uri);
        let token = device_code::get_token(&client, data).await.unwrap();
        dbg!("Token getting successfully!\nAccess Token: {}\nRefresh Token: {}\nExpires in: {}", &token.access_token, &token.refresh_token, &token.expires_in);
        let xbox_live_res = common::xbox_live_auth(&client, token).await.unwrap();
        dbg!("Xbox Live Authenticate successfully!\nToken: {}\nuhs: {}\n", &xbox_live_res.token, &xbox_live_res.display_claims.xui[0].uhs);
        let xsts_res = common::xsts_auth(&client, xbox_live_res).await.unwrap();
        dbg!("XSTS Authenticate successfully!\nToken: {}\nuhs: {}\n", &xsts_res.token, &xsts_res.display_claims.xui[0].uhs);
        let mc_res = common::get_mc_access_token(&client, xsts_res).await.unwrap();
        dbg!("Minecraft Authenticate successfully!\nAccess Token: {}\nExpires in: {}\n", &mc_res.access_token, &mc_res.expires_in);
    }
}