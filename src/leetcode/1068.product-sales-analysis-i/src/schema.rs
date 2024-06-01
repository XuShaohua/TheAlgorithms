// @generated automatically by Diesel CLI.

diesel::table! {
    product (product_id) {
        product_id -> Int4,
        #[max_length = 10]
        product_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    sales (sale_id, year) {
        sale_id -> Int4,
        product_id -> Int4,
        year -> Int4,
        quantity -> Int4,
        price -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    product,
    sales,
);
