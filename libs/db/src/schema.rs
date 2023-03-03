diesel::table! {
    devices (id) {
        id -> Int8,
        external_id -> Nullable<Int2>,
        name -> Nullable<Varchar>,
        status -> Int2,
        kind -> Int2,
        greenhouse_id -> Int8,
        created_at -> Timestamp,
    }
}

diesel::table! {
    greenhouses (id) {
        id -> Int8,
        name -> Varchar,
        token -> Varchar,
        owner_id -> Int8,
        created_at -> Timestamp,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int8,
        token -> Varchar,
        user_id -> Nullable<Int8>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        email -> Varchar,
        password_hash -> Text,
        username -> Varchar,
        created_at -> Timestamp,
        locale -> Varchar,
        theme -> Int2,
    }
}

diesel::joinable!(devices -> greenhouses (greenhouse_id));
diesel::joinable!(greenhouses -> users (owner_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    devices,
    greenhouses,
    sessions,
    users,
);
