use std::time::Duration;

use failure::Error;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder,
};
use serde::Deserialize;

use crate::get_ctf_id_from_name;

#[derive(Debug)]
pub struct CTFDServiceConfig {
    pub name: String,
    pub base_url: String,
    pub api_url: String,
    pub api_key: String,
}

pub struct CTFDService {
    pub id: i32,
    pub config: CTFDServiceConfig,
    client: Client,
}

#[derive(Debug, Deserialize)]
struct GetChallengesResponse {
    data: Vec<ChallengeResponse>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct ChallengeResponse {
    pub name: String,
    pub value: i32,
    pub solves: Option<i32>,
    pub category: String,
}

#[derive(Debug, Deserialize)]
pub struct GetTeamSolvesResponse {
    pub data: Vec<TeamSolvesResponseData>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct TeamSolvesResponseData {
    pub date: String,
    pub team: i32,
    pub challenge: ChallengeResponse,
    pub user: i32,
}

#[derive(Debug, Deserialize)]
pub struct GetUserByIdResponse {
    data: UserResponseData,
}

#[derive(Debug, Deserialize)]
pub struct UserResponseData {
    pub name: String,
    pub score: i32,
}

#[derive(Debug, Deserialize)]
pub struct MyTeamResponse {
    data: MyTeamResponseData,
}

#[derive(Debug, Deserialize)]
pub struct MyTeamResponseData {
    pub place: String,
    pub score: i32,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialise_stats() {
        let data = r#"{"success": true, "data": {"affiliation": null, "members": [62, 63, 77, 474, 854, 900, 1397], "name": "purple_ctf", "id": 23, "fields": [], "bracket": null, "oauth_id": null, "website": null, "country": null, "captain_id": 62, "email": null, "place": "96th", "score": 201}}"#;

        let my_team_resp: MyTeamResponse = serde_json::from_str(data).unwrap();

        assert_eq!(my_team_resp.data.score, 201);
    }

    #[test]
    fn test_deserialise_get_user_from_id() {
        let data = r#"
        {
    "success": true,
    "data": {
        "affiliation": null,
        "bracket": null,
        "website": null,
        "id": 12,
        "oauth_id": null,
        "team_id": 4,
        "country": null,
        "name": "Craig",
        "fields": [],
        "place": "25th",
        "score": 5
    }
}"#;

        let get_user_by_id_resp: GetUserByIdResponse =
            serde_json::from_str(data).expect("Err on deserialising response");

        assert_eq!(get_user_by_id_resp.data.name, "Craig".to_string());
    }

    #[test]
    fn test_get_my_team_solves() {
        let data = r#"
  {
    "success": true,
    "data": [
        {
            "type": "correct",
            "date": "2021-05-13T11:01:54+00:00",
            "team": 10,
            "challenge": {
                "name": "Reverse a String",
                "value": 100,
                "category": "Programming"
            },
            "user": 52,
            "id": 249,
            "challenge_id": 4
        }
    ]
}
"#;

        let get_my_team_solves: GetTeamSolvesResponse =
            serde_json::from_str(data).expect("Err on deserialising response");

        assert_eq!(get_my_team_solves.data.len(), 1);
    }
}
