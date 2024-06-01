// @generated automatically by Diesel CLI.

diesel::table! {
    orders (order_number) {
        order_number -> Int4,
        customer_number -> Int4,
    }
}
