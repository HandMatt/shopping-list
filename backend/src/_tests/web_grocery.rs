use super::grocery_rest_filters;
use crate::model::{init_db, Grocery, GroceryStatus};
use crate::web::handle_rejection;
use anyhow::{Context, Ok, Result};
use serde::Deserialize;
use serde_json::{from_str, from_value, Value};
use std::str::from_utf8;
use std::sync::Arc;
use warp::hyper::body::Bytes;
use warp::hyper::Response;
use warp::Filter;

/// Test grocery list
#[tokio::test]
async fn web_grocery_list() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let grocery_apis = grocery_rest_filters("api", db.clone()).recover(handle_rejection);

    // -- ACTION
    let resp = warp::test::request()
        .method("GET")
        .header("X-Auth-Token", "123")
        .path("/api/groceries")
        .reply(&grocery_apis)
        .await;

    // -- CHECK
    assert_eq!(200, resp.status(), "http status");

    // extract response .data
    let groceries: Vec<Grocery> = extract_body_data(resp)?;

    // -- CHECK - groceries
    assert_eq!(2, groceries.len(), "number of groceries");
    assert_eq!(101, groceries[0].id);
    assert_eq!("orange", groceries[0].name);
    assert_eq!(GroceryStatus::Shelf, groceries[0].status);

    Ok(())
}

// region:    Web Test Utils
/// Extract the data from the response
fn extract_body_data<D>(resp: Response<Bytes>) -> Result<D>
where
    for<'de> D: Deserialize<'de>,
{
    // parse the body as serde_json::Value
    let body = from_utf8(resp.body())?;
    let mut body: Value = from_str(body)
        .with_context(|| format!("Cannot parse resp.body to JSON. resp.body: '{}'", body))?;

    // extract the data
    let data = body["data"].take();

    // deserialize the data to D
    let data: D = from_value(data)?;

    Ok(data)
}
// endregion: Web Test Utils
