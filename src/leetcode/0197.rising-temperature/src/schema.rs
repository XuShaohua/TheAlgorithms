// @generated automatically by Diesel CLI.

diesel::table! {
    weather (id) {
        id -> Int4,
        recorddate -> Date,
        temperature -> Int4,
    }
}
