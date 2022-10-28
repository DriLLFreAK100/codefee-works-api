// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        status -> Int2,
    }
}
