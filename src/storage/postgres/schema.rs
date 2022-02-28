table! {
    task_logs (task_id, user_id) {
        task_id -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
    }
}

joinable!(task_logs -> tasks (task_id));
joinable!(task_logs -> users (user_id));

allow_tables_to_appear_in_same_query!(task_logs, tasks, users,);
