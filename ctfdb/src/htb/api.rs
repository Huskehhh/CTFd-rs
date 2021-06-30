use std::time::Duration;

use failure::Error;
use reqwest::{Client, ClientBuilder};
use serde_json::json;

use crate::create_reqwest_client;

use super::structs::*;

pub static API_URL: &str = "https://www.hackthebox.eu/api/v4";

pub async fn new_htbapi_instance(config: HTBAPIConfig) -> Result<HTBApi, Error> {
    let login_client = ClientBuilder::new()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Error when creating login reqwest client");

    let token = login_and_get_token(&config, &login_client).await?;

    let client = create_reqwest_client(&token, "Bearer");

    Ok(HTBApi { config, client })
}

async fn login_and_get_token(config: &HTBAPIConfig, client: &Client) -> Result<String, Error> {
    let url = format!("{}/login", API_URL);

    let login_post_data =
        json!({"email": config.email, "password": config.password, "remember": true});

    let login_response = client
        .post(&url)
        .json(&login_post_data)
        .send()
        .await?
        .json::<LoginResponse>()
        .await?;

    Ok(login_response.message.access_token)
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

    pub async fn get_recent_team_activity(&self) -> Result<Vec<GetRecentTeamActivityData>, Error> {
        let url = format!("{}/team/activity/{}", API_URL, &self.config.team_id);

        let team_members = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<Vec<GetRecentTeamActivityData>>()
            .await?;

        Ok(team_members)
    }

    pub async fn get_challenge_categories(&self) -> Result<ListChallengeCategories, Error> {
        let url = format!("{}/challenge/categories/list", API_URL);

        let challenge_categories = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<ListChallengeCategories>()
            .await?;

        Ok(challenge_categories)
    }
}
