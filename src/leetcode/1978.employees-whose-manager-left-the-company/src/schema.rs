// @generated automatically by Diesel CLI.

diesel::table! {
    employees (id) {
        id -> Int4,
        employee_id -> Int4,
        #[max_length = 20]
        name -> Varchar,
        manager_id -> Nullable<Int4>,
        salary -> Int4,
    }
}
