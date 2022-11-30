// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        draft -> Bool,
        published_at -> Timestamp,
        visit_count -> Int4,
    }
}
