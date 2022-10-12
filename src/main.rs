use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

use clap::Parser;
use schema::worklog;

use crate::models::NewWorkItem;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    description: String,
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn display_work_items() {
    use self::schema::worklog::dsl::*;
    use models::WorkItem;

    let connection = &mut establish_connection();
    let results = worklog
        .load::<WorkItem>(connection)
        .expect("Error loading work items.");

    println!("Displaying {} work items", results.len());
    for work_item in results {
        println!("{}: {}", work_item.created_at, work_item.description);
    }
}

fn create_work_item(description: &str) {
    let work_item = NewWorkItem { description };

    let connection = &mut establish_connection();
    diesel::insert_into(worklog::table)
        .values(work_item)
        .execute(connection)
        .expect("Error saving work item");

    println!("Work item added.")
}

fn main() {
    let args = Args::parse();

    create_work_item(&args.description);
    display_work_items();
}
