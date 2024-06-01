// @generated automatically by Diesel CLI.

diesel::table! {
    customer (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        referee_id -> Nullable<Int4>,
    }
}
