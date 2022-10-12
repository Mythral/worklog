use crate::schema;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = schema::worklog)]
pub struct NewWorkItem<'a> {
    pub description: &'a str,
}

#[derive(Queryable)]
pub struct WorkItem {
    pub id: i32,
    pub created_at: String,
    pub description: String,
}

#[derive(Queryable)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct TagKeyword {
    pub id: i32,
    pub tag_id: i32,
    pub keyword: String,
}

#[derive(Queryable)]
pub struct WorkTag {
    pub work_id: i32,
    pub tag_id: i32,
}
