use reqwest::Client;

use crate::{
    errors::Errors,
    models::discord::{AccessToken, UserData},
};

pub struct DiscordService {
    client: Client,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    access: Option<AccessToken>,
}

impl DiscordService {
    pub fn new<T: AsRef<str>>(client_id: T, client_secret: T, redirect_uri: T) -> Self {
        let client = Client::new();
        Self {
            client,
            client_id: client_id.as_ref().to_string(),
            client_secret: client_secret.as_ref().to_string(),
            redirect_uri: redirect_uri.as_ref().to_string(),
            access: None,
        }
    }

    pub async fn get_token(&mut self, code: &str) -> Result<AccessToken, Errors> {
        let request = self
            .client
            .post("https://discordapp.com/api/oauth2/token")
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("grant_type", "authorization_code"),
                ("redirect_uri", self.redirect_uri.as_str()),
                ("code", code),
            ])
            .build()?;

        let access = self.client.execute(request).await?;

        let access = access.json::<AccessToken>().await?;
        self.access = Some(access.clone());

        Ok(access)
    }

    pub async fn get_user(&self) -> Result<UserData, Errors> {
        let access_token = match &self.access {
            Some(access) => &access.access_token,
            None => return Err(Errors::InvalidSession),
        };

        let user_api_url = "https://discord.com/api/users/@me";
        let client = Client::new();
        let user_response = client.get(user_api_url).bearer_auth(access_token).send().await?;
        let user = user_response.json::<UserData>().await?;
        Ok(user)
    }

    pub async fn get_user_with_access_token(access_token: &str) -> Result<UserData, Errors> {
        let user_api_url = "https://discord.com/api/users/@me";
        let client = Client::new();
        let user_response = client.get(user_api_url).bearer_auth(access_token).send().await?;
        let user = user_response.json::<UserData>().await?;
        Ok(user)
    }
}
