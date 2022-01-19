table! {
    post (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        tag_id -> Uuid,
        title -> Text,
        body -> Text,
    }
}

table! {
    tag (id) {
        id -> Uuid,
        user_id -> Nullable<Uuid>,
        title -> Text,
    }
}

table! {
    user (id) {
        id -> Uuid,
        email -> Text,
    }
}

joinable!(post -> tag (tag_id));
joinable!(post -> user (user_id));
joinable!(tag -> user (user_id));

allow_tables_to_appear_in_same_query!(
    post,
    tag,
    user,
);
