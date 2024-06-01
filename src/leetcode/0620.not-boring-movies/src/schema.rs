// @generated automatically by Diesel CLI.

diesel::table! {
    cinema (id) {
        id -> Int4,
        #[max_length = 255]
        movie -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        rating -> Nullable<Numeric>,
    }
}
