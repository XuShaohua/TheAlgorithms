// @generated automatically by Diesel CLI.

diesel::table! {
    employee (id) {
        id -> Int4,
        #[max_length = 32]
        name -> Nullable<Varchar>,
        salary -> Nullable<Int4>,
        managerid -> Nullable<Int4>,
    }
}
