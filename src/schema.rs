table! {
    chats (id) {
        id -> Uuid,
        user_name -> Varchar,
        body -> Text,
        ts -> Timestamp,
    }
}
