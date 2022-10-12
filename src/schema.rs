// @generated automatically by Diesel CLI.

diesel::table! {
    tag_keywords (id) {
        id -> Integer,
        keyword -> Text,
        tag_id -> Integer,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    work_tags (work_id, tag_id) {
        work_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    worklog (id) {
        id -> Integer,
        created_at -> Text,
        description -> Text,
    }
}

diesel::joinable!(tag_keywords -> tags (tag_id));
diesel::joinable!(work_tags -> tags (tag_id));
diesel::joinable!(work_tags -> worklog (work_id));

diesel::allow_tables_to_appear_in_same_query!(
    tag_keywords,
    tags,
    work_tags,
    worklog,
);
