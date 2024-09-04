#![allow(unused)] // silence unused warnings while exploring (to comment out)

use std::sync::Arc;
use model::init_db;
mod model;

#[tokio::main]
async fn main() {
    // get the database
    // TODO - loop until valid DB
    let db = init_db().await.expect("Cannot init db");
    let db = Arc::new(db);
}
