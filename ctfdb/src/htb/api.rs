use std::time::Duration;

use failure::Error;
use reqwest::{Client, ClientBuilder};
use serde_json::json;

use crate::{create_reqwest_client, jwt_still_valid};

use super::structs::*;

pub static API_URL: &str = "https://www.hackthebox.eu/api/v4";

pub async fn new_htbapi_instance(config: HTBAPIConfig) -> Result<HTBApi, Error> {
    let login_client = ClientBuilder::new()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("Error when creating login reqwest client");

    let token = login_and_get_token(&config, &login_client).await?;

    let client = create_reqwest_client(&token, "Bearer");

    let jwt = parse_jwt(&token)?;

    Ok(HTBApi {
        config,
        client,
        jwt,
    })
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

    pub async fn list_active_machines(&self) -> Result<ListActiveMachines, Error> {
        let url = format!("{}/machine/list", API_URL);

        let active_machines = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<ListActiveMachines>()
            .await?;
        Ok(active_machines)
    }

    pub async fn list_team_members(&self) -> Result<Vec<ListTeamMembersData>, Error> {
        let url = format!("{}/team/members/{}", API_URL, &self.config.team_id);

        let team_members = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<Vec<ListTeamMembersData>>()
            .await?;

        Ok(team_members)
    }

    pub async fn get_team_statistics(&self) -> Result<GetTeamStatistics, Error> {
        let url = format!("{}/team/stats/owns/{}", API_URL, &self.config.team_id);

        let team_stats = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<GetTeamStatistics>()
            .await?;

        Ok(team_stats)
    }

    pub async fn get_team_rank(&self) -> Result<RankStats, Error> {
        let url = format!("{}/rankings/team/ranking_bracket", API_URL);

        let team_rank = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<RankStats>()
            .await?;

        Ok(team_rank)
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

    pub async fn get_user_activity(&self, user_id: i32) -> Result<UserActivity, Error> {
        let url = format!("{}/user/profile/activity/{}", API_URL, user_id);

        let users_recent_activity = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<UserActivity>()
            .await?;

        Ok(users_recent_activity)
    }

    pub async fn handle_token_renewal(&mut self) -> Result<(), Error> {
        if !jwt_still_valid(&self.jwt) {
            let token = login_and_get_token(&self.config, &self.client).await?;

            self.jwt = parse_jwt(&token)?;
            self.client = create_reqwest_client(&token, "Bearer");
        }

        Ok(())
    }
}
