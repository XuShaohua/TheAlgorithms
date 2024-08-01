// @generated automatically by Diesel CLI.

diesel::table! {
    teacher (id) {
        id -> Int4,
        teacher_id -> Int4,
        subject_id -> Int4,
        dept_id -> Int4,
    }
}
