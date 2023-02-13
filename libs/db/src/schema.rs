diesel::table! {
    sessions (id) {
        id -> Int8,
        token -> Varchar,
        user_id -> Nullable<Int8>,
        created_at -> Timestamp,
    }
}
