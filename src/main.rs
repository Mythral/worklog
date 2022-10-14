pub mod db;
pub mod models;
pub mod schema;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        work_item: String,
    },
    AddTag {
        tag_name: String,
        keywords: Vec<String>,
    },
    List {},
}

fn main() {
    let args = Args::parse();
    match args.command {
        Commands::Add { work_item } => {
            db::create_work_item(&work_item);
        }
        Commands::AddTag { tag_name, keywords } => {
            db::create_tag(&tag_name, keywords.iter().map(|s| s.as_ref()).collect());
        }
        Commands::List {} => {
            db::display_work_items();
            println!();
            db::display_tags();
        }
    }
}
