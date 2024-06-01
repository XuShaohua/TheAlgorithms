// @generated automatically by Diesel CLI.

diesel::table! {
    courses (student, class) {
        #[max_length = 255]
        student -> Varchar,
        #[max_length = 255]
        class -> Varchar,
    }
}
