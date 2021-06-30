use reqwest::Client;
use serde::Deserialize;

// All information from https://github.com/Propolisa/htb-api-docs

#[derive(Debug, Deserialize)]
pub struct GetTeamProfile {
    pub id: i32,
    pub name: String,
    pub points: i32,
}

#[derive(Debug, Deserialize)]
pub struct GetRecentTeamActivityData {
    pub user: UserData,
    pub date: String,
    #[serde(rename = "type")]
    pub type_str: String,
    pub object_type: String,
    pub id: i32,
    pub name: String,
    pub points: i32,
    pub challenge_category: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub name: String,
    pub avatar_thumb: String,
}

#[derive(Debug, Deserialize)]
pub struct GetTeamStatistics {
    pub rank: i32,
    pub user_owns: i32,
    pub system_owns: i32,
}

#[derive(Debug, Deserialize)]
pub struct ListTeamMembers {
    pub data: Vec<ListTeamMembersData>,
}

#[derive(Debug, Deserialize)]
pub struct ListTeamMembersData {
    id: i32,
    name: String,
    rank: i32,
    points: i32,
    root_owns: i32,
    user_owns: i32,
    rank_text: String,
}

#[derive(Debug, Deserialize)]
pub struct ListActiveChallenges {
    pub challenges: Vec<ListActiveChallengesData>,
}

#[derive(Debug, Deserialize)]
pub struct ListActiveChallengesData {
    pub id: i32,
    pub name: String,
    pub difficulty: String,
    pub points: String, // TODO Double check this...
    pub release_date: String,
    pub challenge_category_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct ListChallengeCategories {
    pub info: Vec<ListChallengeCategoriesData>,
}

#[derive(Debug, Deserialize)]
pub struct ListChallengeCategoriesData {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub message: LoginResponseData,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponseData {
    pub access_token: String,
}

#[derive(Debug)]
pub struct HTBAPIConfig {
    pub email: String,
    pub password: String,
    pub team_id: i32,
}

#[derive(Debug)]
pub struct HTBApi {
    pub config: HTBAPIConfig,
    pub client: Client,
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use super::*;

    fn read_file_to_string(filename: &str) -> String {
        let mut base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        base.push("resources/test");
        base.push(filename);

        fs::read_to_string(base).unwrap()
    }

    #[test]
    fn test_deserialise_recent_activity() {
        let data = read_file_to_string("recent_actiity.json");

        let recent_data: Vec<GetRecentTeamActivityData> = serde_json::from_str(&data).unwrap();

        assert_eq!(recent_data.len(), 2);
        assert_eq!(recent_data[0].name, "Missing in Action".to_string());
        assert_eq!(recent_data[0].user.name, "wulfgarpro".to_string());

        assert_eq!(recent_data[1].name, "Ophiuchi".to_string());
        assert_eq!(recent_data[1].object_type, "machine".to_string());
        assert!(recent_data[1].challenge_category.is_none());
    }

    #[test]
    fn test_deserialise_list_challenges() {
        let data = read_file_to_string("list_challenges.json");

        let active_challenges: ListActiveChallenges = serde_json::from_str(&data).unwrap();
        let challenges = active_challenges.challenges;

        assert_ne!(challenges.len(), 0);
        assert_eq!(challenges[0].name, "Bombs Landed");
    }

    #[test]
    fn test_deserialise_challenge_categories() {
        let data = read_file_to_string("challenge_categories.json");

        let challenge_categories: ListChallengeCategories = serde_json::from_str(&data).unwrap();
        let categories = challenge_categories.info;

        assert_ne!(categories.len(), 0);
        assert_eq!(categories[0].id, 1);
        assert_eq!(categories[0].name, "Reversing");
    }

    #[test]
    fn test_deserialise_login_response() {
        let data = read_file_to_string("login_response.json");

        let login_response: LoginResponse = serde_json::from_str(&data).unwrap();

        assert_eq!(login_response.message.access_token, "abcd");
    }
}
