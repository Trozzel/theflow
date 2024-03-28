// @generated automatically by Diesel CLI.

diesel::table! {
    contexts (unique_id) {
        unique_id -> Bigint,
        name -> Text,
        parent_id -> Nullable<Bigint>,
        #[max_length = 7]
        status -> Char,
        created -> Datetime,
        modified -> Datetime,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    folders (unique_id) {
        unique_id -> Bigint,
        name -> Text,
        parent_id -> Nullable<Bigint>,
        #[max_length = 7]
        status -> Char,
        created -> Datetime,
        modified -> Datetime,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    projects (unique_id) {
        unique_id -> Bigint,
        name -> Text,
        parent_id -> Nullable<Bigint>,
        #[max_length = 7]
        status -> Char,
        created -> Datetime,
        modified -> Datetime,
        notes -> Nullable<Text>,
        context_id -> Nullable<Bigint>,
        folder_id -> Nullable<Bigint>,
        flagged -> Bool,
        deferred -> Nullable<Datetime>,
        due -> Nullable<Datetime>,
        is_repeating -> Bool,
        #[max_length = 8]
        repeat_from -> Char,
        #[max_length = 50]
        repeat_schedule -> Char,
        complete_with_last -> Bool,
        #[max_length = 50]
        review_schedule -> Char,
        #[max_length = 12]
        project_type -> Char,
    }
}

diesel::table! {
    tasks (unique_id) {
        unique_id -> Bigint,
        name -> Text,
        parent_id -> Nullable<Bigint>,
        #[max_length = 7]
        status -> Char,
        created -> Datetime,
        modified -> Datetime,
        notes -> Nullable<Text>,
        context_id -> Nullable<Bigint>,
        project_id -> Nullable<Bigint>,
        flagged -> Bool,
        deferred -> Nullable<Datetime>,
        due -> Nullable<Datetime>,
        is_repeating -> Bool,
        #[max_length = 8]
        repeat_from -> Char,
        #[max_length = 50]
        repeat_schedule -> Char,
        #[max_length = 10]
        task_type -> Char,
    }
}

diesel::joinable!(projects -> contexts (context_id));
diesel::joinable!(projects -> folders (folder_id));
diesel::joinable!(tasks -> contexts (context_id));
diesel::joinable!(tasks -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(
    contexts,
    folders,
    projects,
    tasks,
);
