// @generated automatically by Diesel CLI.

diesel::table! {
    actordirector (timestamp) {
        actor_id -> Int4,
        director_id -> Int4,
        timestamp -> Int4,
    }
}
