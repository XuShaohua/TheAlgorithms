// @generated automatically by Diesel CLI.

diesel::table! {
    prices (id) {
        id -> Int4,
        product_id -> Int4,
        start_date -> Date,
        end_date -> Date,
        price -> Int4,
    }
}

diesel::table! {
    unitssold (id) {
        id -> Int4,
        product_id -> Int4,
        purchase_date -> Date,
        units -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    prices,
    unitssold,
);
