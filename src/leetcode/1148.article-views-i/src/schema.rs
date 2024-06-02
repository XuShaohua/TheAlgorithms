// @generated automatically by Diesel CLI.

diesel::table! {
    views (id) {
        id -> Int4,
        article_id -> Int4,
        author_id -> Int4,
        viewer_id -> Int4,
        view_date -> Date,
    }
}
