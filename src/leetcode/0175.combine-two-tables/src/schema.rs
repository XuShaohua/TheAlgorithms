// @generated automatically by Diesel CLI.

diesel::table! {
    address (addressid) {
        addressid -> Int4,
        personid -> Int4,
        #[max_length = 64]
        city -> Nullable<Varchar>,
        #[max_length = 64]
        state -> Nullable<Varchar>,
    }
}

diesel::table! {
    person (personid) {
        personid -> Int4,
        #[max_length = 32]
        lastname -> Varchar,
        #[max_length = 32]
        firstname -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    address,
    person,
);
