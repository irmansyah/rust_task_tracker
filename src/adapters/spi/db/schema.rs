diesel::table! {
    tasks (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        typ -> Nullable<Text>,
        priority -> Nullable<Text>,
        status -> Nullable<Text>,
        description -> Nullable<Text>,
        duration -> Nullable<Int4>,
        due_date -> Nullable<BigInt>,
        project_id -> Nullable<Int4>,
        task_list ->  Nullable<Array<Text>>,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

joinable!(tasks -> projects (project_id));

allow_tables_to_appear_in_same_query!(
    tasks,
    projects,
);
