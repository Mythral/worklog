pub mod db;
pub mod models;
pub mod schema;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    description: String,
}

fn main() {
    let args = Args::parse();

    db::create_work_item(&args.description);
    db::display_work_items();
}
