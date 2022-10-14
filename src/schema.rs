// @generated automatically by Diesel CLI.

diesel::table! {
    tag_keywords (id) {
        id -> Int4,
        keyword -> Varchar,
        tag_id -> Int4,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    work_tags (work_id, tag_id) {
        work_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    worklog (id) {
        id -> Int4,
        created_at -> Timestamp,
        description -> Varchar,
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
