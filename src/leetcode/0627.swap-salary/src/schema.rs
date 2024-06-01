// @generated automatically by Diesel CLI.

diesel::table! {
    salary (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 1]
        sex -> Bpchar,
        salary -> Int4,
    }
}
