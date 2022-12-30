// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        status -> Int2,
        tags -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    todos_relations (id) {
        id -> Int4,
        parent_todo_id -> Nullable<Int4>,
        child_todo_id -> Nullable<Int4>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    todos,
    todos_relations,
);
