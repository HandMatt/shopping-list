use super::GroceryMac;
use crate::model;
use crate::model::db::init_db;
use crate::model::grocery::{Grocery, GroceryPatch, GroceryStatus};
use crate::security::utx_from_token;

/// Test grocery create
#[tokio::test]
async fn model_grocery_create() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;
    let data_fx = GroceryPatch {
        name: Some("test - model_grocery_create 1".to_string()),
        cost: Some(0),
        ..Default::default()
    };

    // -- ACTION
    let grocery_created = GroceryMac::create(&db, &utx, data_fx.clone()).await?;

    // -- CHECK
    assert!(grocery_created.id >= 1000, "Id should be >= 1000");
    assert_eq!(data_fx.name.unwrap(), grocery_created.name);
    assert_eq!(GroceryStatus::Shelf, grocery_created.status);

    Ok(())
}

/// Test grocery get ok
#[tokio::test]
async fn model_grocery_get_ok() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;

    // -- ACTION
    let grocery = GroceryMac::get(&db, &utx, 100).await?;

    // -- CHECK
    assert_eq!(100, grocery.id);
    assert_eq!("banana", grocery.name);
    assert_eq!(GroceryStatus::Basket, grocery.status);

    Ok(())
}

/// Test grocery get wrong id
#[tokio::test]
async fn model_grocery_get_wrong_id() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;

    // -- ACTION
    let result = GroceryMac::get(&db, &utx, 999).await;

    // -- CHECK
    match result {
        Ok(_) => panic!("Should not succeed"),
        Err(model::Error::EntityNotFound(typ, id)) => {
            assert_eq!("groceries", typ);
            assert_eq!(999.to_string(), id);
        }
        other_error => panic!("Wrong Error {:?} ", other_error),
    }

    Ok(())
}

/// Test grocery list
#[tokio::test]
async fn model_grocery_list() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;

    // -- ACTION
    let groceries = GroceryMac::list(&db, &utx).await?;

    // -- CHECK
    assert_eq!(2, groceries.len());
    // println!("\n\n->> {:?}", groceries);
    // grocery 101
    assert_eq!(101, groceries[0].id);
    assert_eq!(123, groceries[0].cid);
    assert_eq!("orange", groceries[0].name);
    assert_eq!(50, groceries[0].cost);
    assert_eq!(GroceryStatus::Shelf, groceries[0].status);
    // grocery 100
    assert_eq!(100, groceries[1].id);
    assert_eq!(123, groceries[1].cid);
    assert_eq!("banana", groceries[1].name);
    assert_eq!(25, groceries[1].cost);
    assert_eq!(GroceryStatus::Basket, groceries[1].status);

    Ok(())
}

/// Test grocery update
#[tokio::test]
async fn model_grocery_update_ok() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;
    let data_fx = GroceryPatch {
        name: Some("test - model_grocery_update_ok 1".to_string()),
        cost: Some(100),
        ..Default::default()
    };
    let grocery_fx = GroceryMac::create(&db, &utx, data_fx.clone()).await?;
    let update_data_fx = GroceryPatch {
        name: Some("test - model_grocery_update_ok 2".to_string()),
        cost: Some(999),
        ..Default::default()
    };

    // -- ACTION
    let grocery_updated =
        GroceryMac::update(&db, &utx, grocery_fx.id, update_data_fx.clone()).await?;

    // -- CHECK
    let groceries = GroceryMac::list(&db, &utx).await?;
    assert_eq!(3, groceries.len());
    assert_eq!(grocery_fx.id, grocery_updated.id);
    assert_eq!(update_data_fx.name.unwrap(), grocery_updated.name);

    Ok(())
}

/// Test grocery delete
#[tokio::test]
async fn model_grocery_delete_simple() -> Result<(), Box<dyn std::error::Error>> {
    // -- FIXTURE
    let db = init_db().await?;
    let utx = utx_from_token(&db, "123").await?;

    // -- ACTION
    let grocery = GroceryMac::delete(&db, &utx, 100).await?;

    // -- CHECK - deleted items
    assert_eq!(100, grocery.id);
    assert_eq!("banana", grocery.name);

    // -- CHECK - list
    let groceries: Vec<Grocery> = sqlb::select().table("groceries").fetch_all(&db).await?;
    assert_eq!(1, groceries.len());

    Ok(())
}
