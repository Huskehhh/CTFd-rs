use reqwest::Client;
use serde::Deserialize;

#[derive(Debug)]
pub enum ChallengeProviderServiceTypes {
    Ctfd,
}
#[derive(Debug)]
pub struct ChallengeProviderServiceConfig {
    pub name: String,
    pub base_url: String,
    pub api_url: String,
    pub api_key: String,
    pub service_type: ChallengeProviderServiceTypes,
}

pub struct CTFDService {
    pub id: i32,
    pub config: ChallengeProviderServiceConfig,
    pub client: Client,
}

#[derive(Debug, Deserialize)]
pub struct GetChallengesResponse {
    pub data: Vec<ChallengeResponse>,
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
    pub data: UserResponseData,
}

#[derive(Debug, Deserialize)]
pub struct UserResponseData {
    pub name: String,
    pub score: i32,
}

#[derive(Debug, Deserialize)]
pub struct MyTeamResponse {
    pub data: MyTeamResponseData,
}

#[derive(Debug, Deserialize)]
pub struct MyTeamResponseData {
    pub place: String,
    pub score: i32,
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
