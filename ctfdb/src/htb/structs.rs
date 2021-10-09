use base64::decode;
use failure::Error;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::models::HTBChallenge;

// All information from https://github.com/Propolisa/htb-api-docs

#[derive(Debug, Deserialize)]
pub struct UserActivity {
    pub profile: UserActivityData,
}

#[derive(Debug, Deserialize)]
pub struct UserActivityData {
    pub activity: Vec<ActivityData>,
}

#[derive(Debug, Deserialize)]
pub struct ActivityData {
    pub date: String,
    pub object_type: String,
    #[serde(rename = "type")]
    pub solve_type: String,
    pub id: i32,
    pub name: String,
    pub points: i32,
}

#[derive(Debug, Deserialize)]
pub struct UserOverview {
    pub profile: UserOverviewData,
}

#[derive(Debug, Deserialize)]
pub struct UserOverviewData {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct GetTeamProfile {
    pub id: i32,
    pub name: String,
    pub points: i32,
}

// This is currently unused, it has been replaced by retrieving all users of the team
// and iterating through their individual activities to get the solve as soon as it happens
#[derive(Debug, Deserialize)]
pub struct GetRecentTeamActivityData {
    pub user: UserData,
    pub date: String,
    #[serde(rename = "type")]
    pub solve_type: String,
    pub object_type: String,
    pub id: i32,
    pub name: String,
    pub points: i32,
    pub challenge_category: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub id: i32,
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
    pub id: i32,
    pub name: String,
    pub rank: i32,
    pub points: i32,
    pub root_owns: i32,
    pub user_owns: i32,
    pub rank_text: String,
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
    pub points: String,
    pub release_date: String,
    pub challenge_category_id: i32,
    pub machine_avatar: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListActiveMachines {
    pub info: Vec<ListActiveMachinesData>,
}

#[derive(Debug, Deserialize)]
pub struct ListActiveMachinesData {
    pub id: i32,
    pub name: String,
    #[serde(rename = "difficultyText")]
    pub difficulty: String,
    pub points: i32,
    pub release: String,
    pub avatar: String,
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

#[derive(Debug, Deserialize)]
pub struct RankStats {
    pub data: RankStatsData,
}

#[derive(Debug, Deserialize)]
pub struct RankStatsData {
    pub rank: i32,
    pub points: i32,
}

#[derive(Debug)]
pub struct SolveToAnnounce {
    pub solver: String,
    pub user_id: i32,
    pub solve_type: String,
    pub challenge: HTBChallenge,
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
    pub jwt: JWTClaims,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTClaims {
    pub exp: i64,
}

pub fn parse_jwt(token: &str) -> Result<JWTClaims, Error> {
    let b64url = token.split('.').collect::<Vec<_>>()[1];
    let buffer = b64url.replace("/-/g", "+").replace("/_/g", "/");
    let decoded = decode(buffer)?[..].to_vec();
    let to_string = String::from_utf8(decoded)?;
    let jwt_claims: JWTClaims = serde_json::from_str(&to_string)?;

    Ok(jwt_claims)
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
        let data = read_file_to_string("recent_activity.json");

        let recent_data: Vec<GetRecentTeamActivityData> = serde_json::from_str(&data).unwrap();

        assert_eq!(recent_data.len(), 2);
        assert_eq!(recent_data[0].name, "Missing in Action".to_string());
        assert_eq!(recent_data[0].user.name, "wulfgarpro".to_string());

        assert_eq!(recent_data[1].name, "Ophiuchi".to_string());
        assert_eq!(recent_data[1].object_type, "machine".to_string());
        assert!(recent_data[1].challenge_category.is_none());
    }

    #[test]
    fn test_deserialise_user_overvoew() {
        let data = read_file_to_string("user_overview.json");

        let recent_data: UserOverview = serde_json::from_str(&data).unwrap();
        let profile = recent_data.profile;

        assert_eq!(508037, profile.id);
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
    fn test_deserialise_list_machines() {
        let data = read_file_to_string("list_machines.json");

        let active_machines: ListActiveMachines = serde_json::from_str(&data).unwrap();
        let machines = active_machines.info;

        assert_ne!(machines.len(), 0);
        assert_eq!(machines[0].name, "RopeTwo");
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

    #[test]
    fn test_deserialise_user_activity() {
        let data = read_file_to_string("get_user_activity.json");

        let get_user_activity: UserActivity = serde_json::from_str(&data).unwrap();

        let activity = &get_user_activity.profile.activity[0];

        assert_eq!(activity.id, 344);
        assert_eq!(activity.solve_type, "root");
        assert_eq!(activity.name, "Love");
    }

    #[test]
    fn test_deserialise_team_members() {
        let data = read_file_to_string("get_team_members.json");

        let team_members: Vec<ListTeamMembersData> = serde_json::from_str(&data).unwrap();

        let member = &team_members[0];

        assert_eq!(member.id, 66487);
        assert_eq!(member.name, "wulfgarpro");
    }

    #[test]
    fn test_deserialise_latest_team_stats() {
        let data = read_file_to_string("latest_team_stats.json");

        let team_stats: RankStats = serde_json::from_str(&data).unwrap();

        assert_eq!(team_stats.data.rank, 381);
        assert_eq!(team_stats.data.points, 101);
    }

    #[test]
    fn test_deserialise_jwt() {
        let token = read_file_to_string("jwt.txt");

        let claims = parse_jwt(&token);

        assert!(claims.is_ok());
    }
}
