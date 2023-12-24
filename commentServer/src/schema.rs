// @generated automatically by Diesel CLI.

diesel::table! {
    comm (id) {
        id -> Int4,
        uuid -> Text,
        in_body -> Text,
    }
}

diesel::table! {
    todos (id) {
        id -> Int4,
        todotext -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    comm,
    todos,
);
