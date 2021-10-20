table! {
    refresh_tokens (id) {
        id -> Text,
        token -> Text,
        user_id -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        email -> Text,
        password_hash -> Text,
    }
}

joinable!(refresh_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(
    refresh_tokens,
    users,
);
