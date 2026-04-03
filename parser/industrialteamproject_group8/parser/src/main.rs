mod db;
mod models;
mod parsers;
mod watcher;

use db::init_db;
use watcher::FileWatcher;

fn main() {
    println!("ATM Parser starting...\n");

    let conn = init_db("sqlite/atm_data.db").expect("Failed to initialize database");
    db::seed_dim_source(&conn).expect("Failed to seed dim_source");

    let mut watcher = FileWatcher::new(conn);
    watcher.watch(5);
}
