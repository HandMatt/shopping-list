use super::grocery_rest_filters;
use crate::model::{init_db, Grocery, GroceryMac, GroceryStatus};
use crate::security::utx_from_token;
use crate::web::handle_rejection;
use anyhow::{Context, Ok, Result};
use serde::Deserialize;
use serde_json::{from_str, from_value, json, Value};
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

/// Test grocery get
#[tokio::test]
async fn web_grocery_get_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let grocery_apis = grocery_rest_filters("api", db).recover(handle_rejection);

    // -- ACTION
    let resp = warp::test::request()
        .method("GET")
        .header("X-Auth-Token", "123")
        .path("/api/groceries/100")
        .reply(&grocery_apis)
        .await;

    // -- CHECK - status
    assert_eq!(200, resp.status(), "http status");

    // extract response .data
    let grocery: Grocery = extract_body_data(resp)?;

    // -- CHECK - .data (grocery)
    assert_eq!(100, grocery.id);
    assert_eq!("banana", grocery.name);
    assert_eq!(25, grocery.cost);
    assert_eq!(GroceryStatus::Basket, grocery.status);

    Ok(())
}

/// Test grocery create
#[tokio::test]
async fn web_grocery_create_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let grocery_apis = grocery_rest_filters("api", db.clone()).recover(handle_rejection);

    // new grocery fixture
    const NAME: &str = "test - web_grocery_create_ok";
    const COST: i64 = 69;
    let body = json!({
        "name": NAME,
        "cost": COST,
    });

    // -- ACTION
    let resp = warp::test::request()
        .method("POST")
        .header("X-Auth-Token", "123")
        .path("/api/groceries")
        .json(&body)
        .reply(&grocery_apis)
        .await;

    // -- CHECK - status
    assert_eq!(200, resp.status(), "http status");

    // extract response .data
    let grocery: Grocery = extract_body_data(resp)?;

    // -- CHECK - .data (grocery)
    assert!(grocery.id >= 1000, "grocery.id should be >= to 1000");
    assert_eq!(NAME, grocery.name);
    assert_eq!(COST, grocery.cost);
    assert_eq!(GroceryStatus::Shelf, grocery.status);

    Ok(())
}

/// Test grocery update
#[tokio::test]
async fn web_grocery_update_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let grocery_apis = grocery_rest_filters("api", db.clone()).recover(handle_rejection);
    // updated grocery
    const NAME: &str = "test - grocery 100 updated";
    const COST: i64 = 5;
    let body = json!({
        "name": NAME,
        "cost": COST,
        "status": "Shelf",
    });

    // -- ACTION
    let resp = warp::test::request()
        .method("PATCH")
        .header("X-Auth-Token", "123")
        .path("/api/groceries/100")
        .json(&body)
        .reply(&grocery_apis)
        .await;

    // -- CHECK - status
    assert_eq!(200, resp.status(), "http status");

    // extract response .data
    let grocery: Grocery = extract_body_data(resp)?;

    // -- CHECK - .data (grocery)
    assert_eq!(100, grocery.id, "grocery.id");
    assert_eq!(NAME, grocery.name);
    assert_eq!(COST, grocery.cost);
    assert_eq!(GroceryStatus::Shelf, grocery.status);

    Ok(())
}

/// Test grocery delete
#[tokio::test]
async fn web_grocery_delete_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let grocery_apis = grocery_rest_filters("api", db.clone()).recover(handle_rejection);

    // -- ACTION
    let resp = warp::test::request()
        .method("DELETE")
        .header("X-Auth-Token", "123")
        .path("/api/groceries/100")
        .reply(&grocery_apis)
        .await;

    // -- CHECK - status
    assert_eq!(200, resp.status(), "http status");

    // extract response .data
    let grocery: Grocery = extract_body_data(resp)?;

    // -- CHECK - .data (grocery)
    assert_eq!(100, grocery.id);
    assert_eq!("banana", grocery.name);
    assert_eq!(25, grocery.cost);
    assert_eq!(GroceryStatus::Basket, grocery.status);

    // -- CHECK - list .len() should be 1
    let utx = utx_from_token(&db, "123").await?;
    let groceries = GroceryMac::list(&db, &utx).await?;
    assert_eq!(1, groceries.len(), "groceries length");
    assert_eq!(101, groceries[0].id, "Grocery remaining should be 101");

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
