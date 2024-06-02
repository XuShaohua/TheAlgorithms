// @generated automatically by Diesel CLI.

diesel::table! {
    department (id, month) {
        id -> Int4,
        revenue -> Int4,
        #[max_length = 5]
        month -> Varchar,
    }
}
