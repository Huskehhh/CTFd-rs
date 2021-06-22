use std::time::Duration;

use failure::Error;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    ClientBuilder,
};

use super::structs::*;

pub static API_URL: &str = "https://www.hackthebox.eu/api/v4";

pub async fn new_htbapi_instance(config: HTBAPIConfig) -> HTBApi {
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

    HTBApi { config, client }
}

impl HTBApi {
    pub async fn list_active_challenges(&self) -> Result<ListActiveChallenges, Error> {
        let url = format!("{}/challenge/list", API_URL);

        let active_challenges = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<ListActiveChallenges>()
            .await?;
        Ok(active_challenges)
    }

    pub async fn list_team_members(&self) -> Result<ListTeamMembers, Error> {
        let url = format!("{}/team/members/{}", API_URL, &self.config.team_id);

        let team_members = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<ListTeamMembers>()
            .await?;

        Ok(team_members)
    }

    pub async fn get_team_statistics(&self) -> Result<GetTeamStatistics, Error> {
        let url = format!("{}/team/stats/owns/{}", API_URL, &self.config.team_id);

        let team_members = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<GetTeamStatistics>()
            .await?;

        Ok(team_members)
    }

    pub async fn get_recent_team_activity(&self) -> Result<GetRecentTeamActivity, Error> {
        let url = format!("{}/team/activity/{}", API_URL, &self.config.team_id);

        let team_members = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<GetRecentTeamActivity>()
            .await?;

        Ok(team_members)
    }
}
