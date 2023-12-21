// @generated automatically by Diesel CLI.

diesel::table! {
    comm (id) {
        id -> Int4,
        uuid -> Text,
        in_body -> Text,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        uuid -> Text,
        body -> Text,
    }
}

diesel::table! {
    commentss (id) {
        id -> Int4,
        uuid -> Text,
        body -> Text,
    }
}

diesel::table! {
    commentsss (id) {
        id -> Int4,
        uuid -> Text,
        in_body -> Text,
    }
}

diesel::table! {
    post (id) {
        id -> Int4,
        body -> Text,
    }
}

diesel::table! {
    todos (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    comm,
    comments,
    commentss,
    commentsss,
    post,
    todos,
);
