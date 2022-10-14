use crate::schema;
use chrono::prelude::*;
use diesel::prelude::*;

#[derive(Insertable)]
#[diesel(table_name = schema::worklog)]
pub struct NewWorkItem<'a> {
    pub description: &'a str,
}

#[derive(Queryable)]
pub struct WorkItem {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::tags)]
pub struct NewTag<'a> {
    pub name: &'a str,
}

#[derive(Queryable)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::tag_keywords)]
pub struct NewTagKeyword<'a> {
    pub tag_id: i32,
    pub keyword: &'a str,
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
