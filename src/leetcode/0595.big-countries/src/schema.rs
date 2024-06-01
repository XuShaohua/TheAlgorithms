// @generated automatically by Diesel CLI.

diesel::table! {
    world (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        continent -> Nullable<Varchar>,
        area -> Nullable<Int4>,
        population -> Nullable<Int4>,
        gdp -> Nullable<Int8>,
    }
}
