diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        role -> Text,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    tasks (id) {
        id -> Int4,
        title -> Text,
        typ -> Nullable<Text>,
        priority -> Nullable<Text>,
        status -> Nullable<Text>,
        description -> Text,
        duration -> Nullable<Int4>,
        due_date -> Nullable<BigInt>,
        project_id -> Nullable<Int4>,
        task_list ->  Nullable<Array<Text>>,
    }
}

joinable!(tasks -> projects (project_id));

allow_tables_to_appear_in_same_query!(
    tasks,
    projects,
);
