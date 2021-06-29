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
pub struct GetRecentTeamActivity {
    pub data: Vec<GetRecentTeamActivityData>
}

#[derive(Debug, Deserialize)]
pub struct GetRecentTeamActivityData {
    pub date: String,
    #[serde(rename = "type")]
    pub type_str: String,
    pub object_type: String,
    pub id: i32,
    pub name: String,
    pub points: i32,
    pub challenge_category: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct GetTeamStatistics {
    pub rank: i32,
    pub user_owns: i32,
    pub system_owns: i32,
}

#[derive(Debug, Deserialize)]
pub struct ListTeamMembers {
    pub data: Vec<ListTeamMembersData>
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
    pub challenges: Vec<ListActiveChallengesData>
}

#[derive(Debug, Deserialize)]
pub struct ListActiveChallengesData {
    pub id: i32,
    pub name: String,
    pub difficulty: String,
    pub points: String, // TODO Double check this...
    pub release_date: String,
    pub challenge_category: i32,
}

#[derive(Debug)]
pub struct HTBAPIConfig {
    pub team_id: i32,
    pub api_key: String,
}

#[derive(Debug)]
pub struct HTBApi {
    pub config: HTBAPIConfig,
    pub client: Client
}