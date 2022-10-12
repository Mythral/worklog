// @generated automatically by Diesel CLI.

diesel::table! {
    tag_keywords (id) {
        id -> Nullable<Integer>,
        keyword -> Nullable<Text>,
        tag_id -> Nullable<Integer>,
    }
}

diesel::table! {
    tags (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    work_tags (work_id, tag_id) {
        work_id -> Nullable<Integer>,
        tag_id -> Nullable<Integer>,
    }
}

diesel::table! {
    worklog (id) {
        id -> Nullable<Integer>,
        created_at -> Nullable<Binary>,
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
