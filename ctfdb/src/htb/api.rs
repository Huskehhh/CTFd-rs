use failure::Error;

use crate::create_reqwest_client;

use super::structs::*;

pub static API_URL: &str = "https://www.hackthebox.eu/api/v4";

pub async fn new_htbapi_instance(config: HTBAPIConfig) -> HTBApi {
    let client = create_reqwest_client(&config.api_key);
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
