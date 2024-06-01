// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        customerid -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    orders,
);
