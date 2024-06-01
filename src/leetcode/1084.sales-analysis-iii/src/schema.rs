// @generated automatically by Diesel CLI.

diesel::table! {
    product (product_id) {
        product_id -> Int4,
        #[max_length = 10]
        product_name -> Varchar,
        unit_price -> Int4,
    }
}

diesel::table! {
    sales (id) {
        id -> Int4,
        seller_id -> Int4,
        product_id -> Int4,
        buyer_id -> Int4,
        sale_date -> Date,
        quantity -> Int4,
        price -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    product,
    sales,
);
