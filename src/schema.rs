// @generated automatically by Diesel CLI.

diesel::table! {
    friends (id) {
        id -> Uuid,
        user_id -> Uuid,
        friend_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        chat_id -> Text,
        phone_number -> Nullable<Text>,
        joined_at -> Timestamp,
        language -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(friends, users,);
