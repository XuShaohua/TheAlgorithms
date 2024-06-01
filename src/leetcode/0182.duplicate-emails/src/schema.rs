// @generated automatically by Diesel CLI.

diesel::table! {
    person (id) {
        id -> Int4,
        #[max_length = 255]
        email -> Varchar,
    }
}
