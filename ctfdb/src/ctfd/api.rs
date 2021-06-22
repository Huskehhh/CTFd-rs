use std::time::Duration;

use failure::Error;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder,
};

use crate::{ctfd::{db::get_ctf_id_from_name, structs::{
        CTFDService, GetChallengesResponse, GetTeamSolvesResponse, GetUserByIdResponse,
        MyTeamResponse,
    }}};

use super::structs::{
    CTFDServiceConfig, ChallengeResponse, MyTeamResponseData, TeamSolvesResponseData,
    UserResponseData,
};

pub async fn new_ctfdservice(config: CTFDServiceConfig) -> CTFDService {
    let mut headers = HeaderMap::new();

    let auth_header = HeaderValue::from_str(&format!("Token {}", &config.api_key))
        .expect("Error creating auth header for new ctfd service");

    let content_type_header = HeaderValue::from_str("application/json")
        .expect("Error when creating content type header for new ctfd service");

    headers.insert("Authorization", auth_header);
    headers.insert("Content-Type", content_type_header);

    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(5))
        .cookie_store(true)
        .default_headers(headers)
        .build()
        .expect("Error when creating reqwest client");

    let ctf_id = get_ctf_id_from_name(&config.name)
        .await
        .expect("Error getting CTF ID from given name");

    CTFDService {
        id: ctf_id,
        config,
        client,
    }
}

impl CTFDService {
    pub async fn get_challenges(&self) -> Result<Vec<ChallengeResponse>, Error> {
        let url = format!("{}/challenges", &self.config.api_url);
        let req = self.client.get(&url).send().await?;
        let response = req.json::<GetChallengesResponse>().await?;
        Ok(response.data)
    }

    pub async fn get_team_solved_challenges(&self) -> Result<Vec<TeamSolvesResponseData>, Error> {
        let url = format!("{}/teams/me/solves", &self.config.api_url);
        let req = self.client.get(&url).send().await?;
        let response = req.json::<GetTeamSolvesResponse>().await?;
        Ok(response.data)
    }

    pub async fn user_from_id(&self, id: i32) -> Result<UserResponseData, Error> {
        let url = format!("{}/users/{}", &self.config.api_url, id);
        let req = self.client.get(&url).send().await?;
        let response = req.json::<GetUserByIdResponse>().await?;
        Ok(response.data)
    }

    pub async fn team_stats(&self) -> Result<MyTeamResponseData, Error> {
        let url = format!("{}/teams/me", &self.config.api_url);
        let req = self.client.get(&url).send().await?;
        let response = req.json::<MyTeamResponse>().await?;
        Ok(response.data)
    }
}
