// @generated automatically by Diesel CLI.

diesel::table! {
    logins (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        time_stamp -> Timestamp,
    }
}
