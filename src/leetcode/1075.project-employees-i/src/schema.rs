// @generated automatically by Diesel CLI.

diesel::table! {
    employee (employee_id) {
        employee_id -> Int4,
        #[max_length = 10]
        name -> Nullable<Varchar>,
        experience_years -> Int4,
    }
}

diesel::table! {
    project (project_id, employee_id) {
        project_id -> Int4,
        employee_id -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    employee,
    project,
);
