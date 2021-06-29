use chrono::NaiveDateTime;

#[derive(Queryable)]
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
    pub points: bool,
    pub release_date: String,
    pub challenge_category: i32,
    pub working: Option<String>,
    pub solved: bool,
    pub solver: Option<String>,
    pub solve_time: Option<NaiveDateTime>,
    pub announced_solve: bool,
}
