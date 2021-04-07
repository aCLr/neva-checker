use crate::result::Result;
use crate::structs::{Contract, Response, TripDetails};
use reqwest::Client as ReqwestClient;
use serde::de::DeserializeOwned;
use tokio::sync::RwLock;

const BASE_URL: &str = "https://api.m11-neva.ru/onyma/system/api";

pub struct Client {
    auth_token: RwLock<Option<String>>,
    client: reqwest::Client,
    login: String,
    password: String,
}

impl Client {
    pub fn new(login: String, password: String) -> Self {
        Self {
            login,
            password,
            client: ReqwestClient::new(),
            auth_token: RwLock::new(None),
        }
    }

    fn build_url(function: &str, arguments: &str) -> String {
        format!(
            "{}/json?realm=M11.S1&function={}&rows_limit=1&{}",
            BASE_URL, function, arguments
        )
    }

    async fn login(&self) -> Result<()> {
        let mut t = self.auth_token.write().await;
        let resp: Response<String> = self
            .client
            .get(&Client::build_url(
                "open_session",
                format!(
                    "user={login}&pass={password}",
                    login = self.login,
                    password = self.password
                )
                .as_str(),
            ))
            .send()
            .await?
            .json()
            .await?;
        *t = Some(resp.r#return());
        Ok(())
    }

    async fn make_request<R: DeserializeOwned>(&self, function: &str) -> Result<R> {
        let token_exists = { self.auth_token.read().await.is_some() };
        if !token_exists {
            self.login().await?;
        }
        let resp = self
            .client
            .get(&Client::build_url(
                function,
                format!(
                    "auth_token={auth_token}",
                    auth_token = self.auth_token.read().await.as_ref().unwrap()
                )
                .as_str(),
            ))
            .send()
            .await?;
        Ok(resp.json::<Response<R>>().await?.r#return())
    }

    pub async fn get_balance(&self) -> Result<Option<Contract>> {
        Ok(self
            .make_request::<Vec<Contract>>("onm_api_crms_api_contract_info")
            .await?
            .pop())
    }

    pub async fn get_last_trip(&self) -> Result<Option<TripDetails>> {
        Ok(self
            .make_request::<Vec<TripDetails>>("onm_api_crms_api_mobile_wall")
            .await?
            .pop())
    }
}
