// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "activity_types"))]
    pub struct ActivityTypes;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ActivityTypes;

    activity (id) {
        id -> Int4,
        user_id -> Int4,
        session_id -> Int4,
        activity_date -> Date,
        activity_type -> Nullable<ActivityTypes>,
    }
}
