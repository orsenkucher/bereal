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
        telegram_id -> Text,
        name -> Nullable<Text>,
        joined_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    friends,
    users,
);
