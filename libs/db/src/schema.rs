diesel::table! {
    device_records (id) {
        id -> Int8,
        device_id -> Int8,
        data -> Float8,
        created_at -> Timestamp,
    }
}

diesel::table! {
    devices (id) {
        id -> Int8,
        external_id -> Nullable<Int2>,
        name -> Nullable<Varchar>,
        status -> Int2,
        kind -> Int2,
        greenhouse_id -> Int8,
        created_at -> Timestamp,
        maximum_data_value -> Nullable<Float8>,
    }
}

diesel::table! {
    greenhouses (id) {
        id -> Int8,
        name -> Varchar,
        token -> Varchar,
        owner_id -> Int8,
        created_at -> Timestamp,
        maximum_average_humidity -> Nullable<Float8>,
        minimum_average_temperature -> Nullable<Float8>,
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

diesel::joinable!(device_records -> devices (device_id));
diesel::joinable!(devices -> greenhouses (greenhouse_id));
diesel::joinable!(greenhouses -> users (owner_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    device_records,
    devices,
    greenhouses,
    sessions,
    users,
);
