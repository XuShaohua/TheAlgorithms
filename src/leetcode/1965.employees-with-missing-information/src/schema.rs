// @generated automatically by Diesel CLI.

diesel::table! {
    employees (id) {
        id -> Int4,
        employee_id -> Int4,
        #[max_length = 30]
        name -> Varchar,
    }
}

diesel::table! {
    salaries (id) {
        id -> Int4,
        employee_id -> Int4,
        salary -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    employees,
    salaries,
);
