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
    pub client: Client,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialise_recent_activity() {
        let data = r#"[
    {
        "user": {
            "id": 66487,
            "name": "wulfgarpro",
            "public": 0,
            "avatar_thumb": "\/storage\/avatars\/2c7844044ac404d3d6bf00ee3e572db6_thumb.png"
        },
        "date": "2021-06-24T22:25:48.000000Z",
        "date_diff": "4 days ago",
        "type": "challenge",
        "first_blood": false,
        "object_type": "challenge",
        "id": 118,
        "name": "Missing in Action",
        "points": 3,
        "challenge_category": "OSINT"
    },
    {
        "user": {
            "id": 66487,
            "name": "wulfgarpro",
            "public": 0,
            "avatar_thumb": "\/storage\/avatars\/2c7844044ac404d3d6bf00ee3e572db6_thumb.png"
        },
        "date": "2021-06-18T11:55:53.000000Z",
        "date_diff": "1 week ago",
        "type": "root",
        "first_blood": false,
        "object_type": "machine",
        "id": 315,
        "name": "Ophiuchi",
        "points": 30,
        "machine_avatar": "\/storage\/avatars\/82b3289bbabf88da886bc9f45802ac17_thumb.png"
    }
    ]"#;

        let recent_data: Vec<GetRecentTeamActivityData> = serde_json::from_str(data).unwrap();

        assert_eq!(recent_data.len(), 2);
        assert_eq!(recent_data[0].name, "Missing in Action".to_string());
        assert_eq!(recent_data[0].user.name, "wulfgarpro".to_string());

        assert_eq!(recent_data[1].name, "Ophiuchi".to_string());
        assert_eq!(recent_data[1].object_type, "machine".to_string());
        assert!(recent_data[1].challenge_category.is_none());
    }
}
