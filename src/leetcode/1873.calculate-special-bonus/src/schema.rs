// @generated automatically by Diesel CLI.

diesel::table! {
    employees (id) {
        id -> Int4,
        employee_id -> Int4,
        #[max_length = 30]
        name -> Varchar,
        salary -> Int4,
    }
}
