diesel::table! {
    dog_facts (id) {
        id -> Int4,
        fact -> Varchar,
    }
}

diesel::table! {
    tasks (id) {
        id -> Int4,
        task -> Varchar,
    }
}
