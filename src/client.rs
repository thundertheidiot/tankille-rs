use chrono::DateTime;
use chrono::Utc;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};
use reqwest::{ClientBuilder, RequestBuilder};
use serde::{Deserialize, Serialize};

use crate::Result;
use crate::TankilleError;

#[derive(Debug)]
pub struct Client {
    refresh_token: Option<Box<str>>,
    access_token: Option<Box<str>>,
    last_token_fetch: Option<DateTime<Utc>>,
    server: Box<str>,
    client: reqwest::Client,
}

#[derive(Serialize)]
pub struct LoginOptions {
    device: Box<str>,
    email: Box<str>,
    password: Box<str>,
}

impl LoginOptions {
    pub fn new(email: &str, password: &str) -> LoginOptions {
        LoginOptions {
            device: "Android SDK built for x86_64 (03280ceb8a5367a6)".into(),
            email: email.into(),
            password: password.into(),
        }
    }
}

impl Client {
    pub fn new() -> Result<Client> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = ClientBuilder::new()
            .user_agent("FuelFellow/3.6.2 (Android SDK built for x86_64; Android 9)")
            .default_headers(headers)
            .build()?;

        Ok(Client {
            access_token: None,
            refresh_token: None,
            last_token_fetch: None,
            server: Box::from("https://api.tankille.fi"),
            client,
        })
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.server, path)
    }

    pub async fn login(&mut self, login_options: LoginOptions) -> Result<()> {
        let response = self
            .client
            .post(self.url("/auth/login"))
            .json(&login_options)
            .send()
            .await?;

        #[derive(Deserialize)]
        struct Response {
            #[serde(rename(deserialize = "refreshToken"))]
            refresh_token: Box<str>,
        }

        let token: Response = response.json().await?;
        self.refresh_token = Some(token.refresh_token);

        Ok(())
    }

    pub fn set_refresh_token(&mut self, token: &str) {
        self.refresh_token = Some(Box::from(token));
    }

    pub fn set_access_token(&mut self, token: &str) {
        self.access_token = Some(Box::from(token));
    }

    pub async fn refresh_token(&mut self) -> Result<()> {
        match &self.refresh_token {
            Some(token) => match self.last_token_fetch {
                // Token expiry time is apparently 12 hours, the library i'm copying uses 10 hours
                Some(t) if (Utc::now() - t).num_seconds() < 36000 => Ok(()),
                _ => {
                    #[derive(Serialize)]
                    struct Data<'a> {
                        #[serde(rename(serialize = "refreshToken"))]
                        token: &'a str,
                    }

                    #[derive(Deserialize)]
                    struct Response {
                        #[serde(rename(deserialize = "accessToken"))]
                        access_token: Box<str>,
                    }

                    let response = self
                        .client
                        .post(self.url("/auth/refresh"))
                        .json(&Data { token })
                        .send()
                        .await?;

                    let token: Response = response.json().await?;
                    self.access_token = Some(token.access_token);
                    self.last_token_fetch = Some(Utc::now());

                    Ok(())
                }
            },
            None => Err(TankilleError::NotAuthenticated),
        }
    }
}
