use chrono::prelude::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::{self, NewTagKeyword, Tag, TagKeyword};
use crate::models::{NewTag, WorkItem};
use crate::schema::{self, tag_keywords, tags};
use models::NewWorkItem;
use schema::worklog;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_tags() -> Vec<Tag> {
    use self::schema::tags::dsl::*;

    let conn = &mut establish_connection();
    tags.load::<Tag>(conn).expect("Error loading tags.")
}

pub fn get_tag_keywords(tag_id_in: i32) -> Vec<String> {
    use self::schema::tag_keywords::dsl::*;

    let conn = &mut establish_connection();

    tag_keywords
        .filter(tag_id.eq(tag_id_in))
        .load::<TagKeyword>(conn)
        .expect("Error loading tag keywords.")
        .into_iter()
        .map(|tag_kw: TagKeyword| tag_kw.keyword)
        .collect()
}

pub fn display_tags() {
    println!("Tags:");
    for tag in get_tags() {
        println!("{}: {:?}", tag.name, get_tag_keywords(tag.id));
    }
}

pub fn display_work_items() {
    use self::schema::worklog::dsl::*;

    let connection = &mut establish_connection();
    let results = worklog
        .load::<WorkItem>(connection)
        .expect("Error loading work items.");

    println!("Displaying {} work items", results.len());
    for work_item in results {
        let creation_datetime: DateTime<Local> = Local.from_utc_datetime(&work_item.created_at);
        println!("{}: {}", creation_datetime, work_item.description);
    }
}

pub fn create_work_item(description: &str) -> WorkItem {
    let work_item = NewWorkItem { description };

    let connection = &mut establish_connection();
    let work_item = diesel::insert_into(worklog::table)
        .values(work_item)
        .get_result::<WorkItem>(connection)
        .expect("Error saving work item");

    println!("Work item added.");

    work_item
}

pub fn create_tag(name: &str, keywords: Vec<&str>) -> Tag {
    let conn = &mut establish_connection();

    let tag = diesel::insert_into(tags::table)
        .values(NewTag { name })
        .get_result::<Tag>(conn)
        .expect("Error creating tag.");

    let keywords: Vec<NewTagKeyword> = keywords
        .into_iter()
        .map(|keyword: &str| -> NewTagKeyword {
            NewTagKeyword {
                tag_id: tag.id,
                keyword,
            }
        })
        .collect();

    diesel::insert_into(tag_keywords::table)
        .values(keywords)
        .execute(conn)
        .expect("created tag keywords");

    tag
}
