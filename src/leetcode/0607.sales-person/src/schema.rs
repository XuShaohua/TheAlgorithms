// @generated automatically by Diesel CLI.

diesel::table! {
    company (com_id) {
        com_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        city -> Nullable<Varchar>,
    }
}

diesel::table! {
    orders (order_id) {
        order_id -> Int4,
        order_date -> Date,
        com_id -> Int4,
        sales_id -> Int4,
        amount -> Int4,
    }
}

diesel::table! {
    salesperson (sales_id) {
        sales_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        salary -> Int4,
        commission_rate -> Nullable<Int4>,
        hire_date -> Nullable<Date>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    company,
    orders,
    salesperson,
);
