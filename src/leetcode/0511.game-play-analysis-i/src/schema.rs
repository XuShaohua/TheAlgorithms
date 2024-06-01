// @generated automatically by Diesel CLI.

diesel::table! {
    activity (player_id, event_date) {
        player_id -> Int4,
        device_id -> Int4,
        event_date -> Date,
        games_played -> Int4,
    }
}
