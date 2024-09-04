use super::GroceryMac;
use crate::model::db::init_db;
use crate::model::grocery::GroceryStatus;
use crate::security::utx_from_token;

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