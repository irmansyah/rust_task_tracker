diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

// diesel::table! {
//     tasks (id) {
//         id -> Int4,
//         title -> Nullable<Varchar>,
//         typ -> Nullable<Text>,
//         priority -> Nullable<Text>,
//         status -> Nullable<Text>,
//         description -> Nullable<Text>,
//         duration -> Nullable<Int4>,
//         due_date -> Nullable<BigInt>,
//         project_id -> Nullable<Int4>,
//         task_list ->  Nullable<Array<Text>>,
//     }
// }

// diesel::table! {
//     tasks (id) {
//         id -> Int4,
//         title -> Text,
//         typ -> Text,
//         priority -> Text,
//         status -> Text,
//         description -> Text,
//         duration -> Int4,
//         due_date -> BigInt,
//         project_id -> Int4,
//         task_list ->  Array<Text>,
//     }
// }

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
