use std::env;

use ctfdb::htb::structs::{HTBAPIConfig, HTBApi};

async fn get_htb_api() -> HTBApi {
    let email = env::var("HTB_EMAIL").expect("No HTB_EMAIL environment variable defined!");
    let password = env::var("HTB_PASSWORD").expect("No HTB_PASSWORD environment variable defined!");
    let team_id = env::var("HTB_TEAM_ID")
        .expect("No HTB_TEAM_ID environment variable defined!")
        .parse::<i32>()
        .expect("HTB_TEAM_ID needs to be an integer!");

    let config = HTBAPIConfig {
        email,
        password,
        team_id,
    };

    let htb_api = ctfdb::htb::api::new_htbapi_instance(config)
        .await
        .expect("Error building HTBApi instance!");

    htb_api
}

#[ignore]
#[tokio::test]
async fn test_get_user_overview() {
    let api = get_htb_api().await;
    let htb_user_id = 508037;

    let result = api.get_user_overview(htb_user_id).await;

    assert!(result.is_ok());

    let user_overview = result.unwrap();
    let user_overview_data = &user_overview.profile;

    assert_eq!(htb_user_id, user_overview_data.id);
}

#[ignore]
#[tokio::test]
async fn test_get_user_activity() {
    let api = get_htb_api().await;
    let htb_user_id = 508037;

    let result = api.get_user_activity(htb_user_id).await;

    assert!(result.is_ok());

    let user_activity = result.unwrap();
    let user_activity_data = &user_activity.profile;

    assert_ne!(0, user_activity_data.activity.len());
}

#[ignore]
#[tokio::test]
async fn test_get_team_recent_activity() {
    let api = get_htb_api().await;

    let result = api.get_recent_team_activity().await;

    assert!(result.is_ok());

    let team_activity = result.unwrap();

    assert_ne!(0, team_activity.len());
}

#[ignore]
#[tokio::test]
async fn test_get_challenge_categories() {
    let api = get_htb_api().await;

    let result = api.get_challenge_categories().await;

    assert!(result.is_ok());

    let list_challenge_categories = result.unwrap();
    let categories = list_challenge_categories.info;

    assert_ne!(0, categories.len());
}

#[ignore]
#[tokio::test]
async fn test_get_team_rank() {
    let api = get_htb_api().await;

    let result = api.get_team_rank().await;

    assert!(result.is_ok());

    let rank_stats = result.unwrap();
    let rank_stats_data = rank_stats.data;

    assert_ne!(0, rank_stats_data.rank);
    assert_ne!(0, rank_stats_data.points);
}

#[ignore]
#[tokio::test]
async fn test_get_team_statistics() {
    let api = get_htb_api().await;

    let result = api.get_team_statistics().await;

    assert!(result.is_ok());

    let team_statistics = result.unwrap();

    assert_ne!(0, team_statistics.rank);
    assert_ne!(0, team_statistics.system_owns);
    assert_ne!(0, team_statistics.user_owns);
}

#[ignore]
#[tokio::test]
async fn test_list_team_members() {
    let api = get_htb_api().await;

    let result = api.list_team_members().await;

    assert!(result.is_ok());

    let team_members = result.unwrap();

    assert_ne!(0, team_members.len());
}

#[ignore]
#[tokio::test]
async fn test_list_active_machines() {
    let api = get_htb_api().await;

    let result = api.list_active_machines().await;

    assert!(result.is_ok());

    let active_machines = result.unwrap();
    let active_machines_data = active_machines.info;

    assert_ne!(0, active_machines_data.len());
}

#[ignore]
#[tokio::test]
async fn test_list_active_challenges() {
    let api = get_htb_api().await;

    let result = api.list_active_challenges().await;

    assert!(result.is_ok());

    let active_challenges = result.unwrap();
    let active_challenges_data = active_challenges.challenges;

    assert_ne!(0, active_challenges_data.len());
}
