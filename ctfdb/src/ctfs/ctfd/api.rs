use failure::Error;

use async_trait::async_trait;

use crate::{
    create_reqwest_client,
    ctfs::{
        db::get_ctf_id_from_name,
        structs::{
            CTFDService, ChallengeProviderServiceConfig, ChallengeResponse, GetChallengesResponse,
            GetTeamSolvesResponse, GetUserByIdResponse, MyTeamResponse, MyTeamResponseData,
            TeamSolvesResponseData, UserResponseData,
        },
    },
    ChallengeProvider,
};

pub async fn new_ctfdservice(
    config: ChallengeProviderServiceConfig,
) -> Box<dyn ChallengeProvider + Send + Sync> {
    let client = create_reqwest_client(&config.api_key, "Token");

    let ctf_id = get_ctf_id_from_name(&config.name)
        .await
        .expect("Error getting CTF ID from given name");

    Box::new(CTFDService {
        id: ctf_id,
        config,
        client,
    })
}

#[async_trait]
impl ChallengeProvider for CTFDService {
    async fn get_challenges(&self) -> Result<Vec<ChallengeResponse>, Error> {
        let url = format!("{}/challenges", &self.config.api_url);
        let req = self.client.get(&url).send().await?;
        let response = req.json::<GetChallengesResponse>().await?;
        Ok(response.data)
    }

    async fn get_team_solved_challenges(&self) -> Result<Vec<TeamSolvesResponseData>, Error> {
        let url = format!("{}/teams/me/solves", &self.config.api_url);
        let req = self.client.get(&url).send().await?;
        let response = req.json::<GetTeamSolvesResponse>().await?;
        Ok(response.data)
    }

    async fn user_from_id(&self, id: i32) -> Result<UserResponseData, Error> {
        let url = format!("{}/users/{}", &self.config.api_url, id);
        let req = self.client.get(&url).send().await?;
        let response = req.json::<GetUserByIdResponse>().await?;
        Ok(response.data)
    }

    async fn team_stats(&self) -> Result<MyTeamResponseData, Error> {
        let url = format!("{}/teams/me", &self.config.api_url);
        let req = self.client.get(&url).send().await?;
        let response = req.json::<MyTeamResponse>().await?;
        Ok(response.data)
    }

    fn get_id(&self) -> i32 {
        self.id
    }
}
