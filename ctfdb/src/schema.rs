table! {
    challenges (id) {
        id -> Integer,
        ctf_id -> Integer,
        name -> Text,
        category -> Text,
        solved -> Bool,
        working -> Nullable<Text>,
        solver -> Nullable<Text>,
        points -> Integer,
        solved_time -> Nullable<Datetime>,
        announced_solve -> Bool,
    }
}

table! {
    ctfs (id) {
        id -> Integer,
        name -> Text,
        base_url -> Text,
        api_url -> Text,
        api_key -> Text,
        channel_id -> Bigint,
        active -> Bool,
    }
}

table! {
    htb_challenges (id) {
        id -> Integer,
        htb_id -> Integer,
        name -> Text,
        difficulty -> Text,
        points -> Text,
        release_date -> Text,
        challenge_category -> Integer,
        working -> Nullable<Text>,
        solved -> Bool,
        solver -> Nullable<Text>,
        solve_time -> Text,
        announced_solve -> Bool,
    }
}

table! {
    scoreboard (entry_id) {
        entry_id -> Integer,
        ctf_id -> Integer,
        points -> Integer,
        position -> Text,
        entry_time -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(
    challenges,
    ctfs,
    htb_challenges,
    scoreboard,
);
