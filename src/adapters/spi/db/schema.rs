diesel::table! {
    tasks (id) {
        id -> Uuid,
        title -> Varchar,
        typ -> Varchar,
        priority -> Varchar,
        status -> Varchar,
        description -> Nullable<Varchar>,
        duration -> Nullable<Int4>,
        due_date -> Nullable<BigInt>,
        project_id -> Nullable<Varchar>,
        task_list ->  Array<Varchar>,
    }
}

diesel::table! {
    projects (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

// joinable!(tasks -> projects (project_id));

allow_tables_to_appear_in_same_query!(
    tasks,
    projects,
);
