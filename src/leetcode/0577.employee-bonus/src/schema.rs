// @generated automatically by Diesel CLI.

diesel::table! {
    bonus (id) {
        id -> Int4,
        empid -> Nullable<Int4>,
        bonus -> Nullable<Int4>,
    }
}

diesel::table! {
    employee (empid) {
        empid -> Int4,
        #[max_length = 32]
        name -> Nullable<Varchar>,
        supervisor -> Nullable<Int4>,
        salary -> Nullable<Int4>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bonus,
    employee,
);
