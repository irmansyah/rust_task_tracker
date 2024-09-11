diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        role -> Text,
        access_token -> VarChar,
        fcm_token -> VarChar,
        last_login -> Timestamp,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    projects (id) {
        id -> Uuid,
        name -> Text,
        description -> Nullable<Text>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    tasks (id) {
        id -> Uuid,
        title -> Text,
        typ -> Nullable<Text>,
        priority -> Nullable<Text>,
        status -> Nullable<Text>,
        description -> Text,
        duration -> Nullable<Int4>,
        due_date -> Nullable<BigInt>,
        project_id -> Nullable<Int4>,
        task_list ->  Nullable<Array<Text>>,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

// joinable!(tasks -> projects (project_id));

allow_tables_to_appear_in_same_query!(
    tasks,
    projects,
);
