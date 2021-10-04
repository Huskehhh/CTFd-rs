use chrono::NaiveDateTime;

#[derive(Debug, Queryable)]
pub struct Ctf {
    pub id: i32,
    pub name: String,
    pub base_url: String,
    pub api_url: String,
    pub api_key: String,
    pub channel_id: i64,
    pub active: bool,
}

#[derive(Debug, Queryable, Clone)]
pub struct Challenge {
    pub id: i32,
    pub ctf_id: i32,
    pub name: String,
    pub category: String,
    pub solved: bool,
    pub working: Option<String>,
    pub solver: Option<String>,
    pub points: i32,
    pub solved_time: Option<NaiveDateTime>,
    pub announced_solve: bool,
}

#[derive(Debug, Queryable, Clone)]
pub struct Scoreboard {
    pub entry_id: i32,
    pub ctf_id: i32,
    pub points: i32,
    pub position: String,
    pub entry_time: NaiveDateTime,
}

#[derive(Debug, Queryable, Clone)]
pub struct HTBChallenge {
    pub id: i32,
    pub htb_id: i32,
    pub name: String,
    pub difficulty: String,
    pub points: String,
    pub release_date: String,
    pub challenge_category: i32,
    pub working: Option<String>,
    pub machine_avatar: Option<String>,
}

#[derive(Debug, Queryable, Clone)]
pub struct HTBSolve {
    pub id: i32,
    pub user_id: i32,
    pub username: String,
    pub challenge_id: i32,
    pub solve_type: String,
    pub announced: bool,
    pub solved_time: NaiveDateTime,
}

#[derive(Debug, Queryable, Clone)]
pub struct HTBRank {
    pub entry_id: i32,
    pub rank: i32,
    pub points: i32,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Queryable)]
pub struct HTBUserMapping {
    pub entry_id: i32,
    pub htb_id: i32,
    pub discord_id: i64,
}
