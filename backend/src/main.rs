#![allow(unused)] // silence unused warnings while exploring (to comment out)

use model::init_db;
use std::sync::Arc;
mod model;
mod security;

/// main - entry point to the application
#[tokio::main]
async fn main() {
    // get the database
    // TODO - loop until valid DB
    let db = init_db().await.expect("Cannot init db");
    let db = Arc::new(db);
}
