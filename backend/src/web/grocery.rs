use super::{filter_auth::do_auth, filter_utils::with_db};
use crate::{
    model::{Db, GroceryMac, GroceryPatch},
    security::UserCtx,
};
use serde::Serialize;
use serde_json::json;
use std::sync::Arc;
use warp::{reply::Json, Filter};

/// grocery REST API
pub fn grocery_rest_filters(
    base_path: &'static str,
    db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let groceries_path = warp::path(base_path).and(warp::path("groceries")); // /api/groceries
    let common = with_db(db.clone()).and(do_auth(db.clone()));

    // LIST groceries `GET groceries/`
    let list = groceries_path
        .and(warp::get())
        .and(warp::path::end())
        .and(common.clone())
        .and_then(grocery_list);

    // GET grocery `GET /groceries/100`
    let get = groceries_path
        .and(warp::get())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(grocery_get);

    // CREATE grocery `POST /groceries with body GroceryPatch`
    let create = groceries_path
        .and(warp::post())
        .and(common.clone())
        .and(warp::body::json())
        .and_then(grocery_create);

    // UPDATE grocery `PATCH /groceries/100 with body GroceryPatch`
    let update = groceries_path
        .and(warp::patch())
        .and(common.clone())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(grocery_update);

    // DELETE grocery `DELETE /groceries/100`
    let delete = groceries_path
        .and(warp::delete())
        .and(common.clone())
        .and(warp::path::param())
        .and_then(grocery_delete);

    list.or(get).or(create).or(update).or(delete)
}

/// GET - `groceries/`
async fn grocery_list(db: Arc<Db>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    let groceries = GroceryMac::list(&db, &utx).await?;
    json_response(groceries)
}

/// GET - `groceries/100`
async fn grocery_get(db: Arc<Db>, utx: UserCtx, id: i64) -> Result<Json, warp::Rejection> {
    let grocery = GroceryMac::get(&db, &utx, id).await?;
    json_response(grocery)
}

/// CREATE - `groceries/` with body `GroceryPatch`
async fn grocery_create(
    db: Arc<Db>,
    utx: UserCtx,
    patch: GroceryPatch,
) -> Result<Json, warp::Rejection> {
    let grocery = GroceryMac::create(&db, &utx, patch).await?;
    json_response(grocery)
}

/// PATCH - `groceries/100` with body `GroceryPatch`
async fn grocery_update(
    db: Arc<Db>,
    utx: UserCtx,
    id: i64,
    patch: GroceryPatch,
) -> Result<Json, warp::Rejection> {
    let grocery = GroceryMac::update(&db, &utx, id, patch).await?;
    json_response(grocery)
}

/// DELETE - `groceries/100`
async fn grocery_delete(db: Arc<Db>, utx: UserCtx, id: i64) -> Result<Json, warp::Rejection> {
    let grocery = GroceryMac::delete(&db, &utx, id).await?;
    json_response(grocery)
}

// region:    Utils
/// json_response
fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
    let response = json!({ "data": data });
    Ok(warp::reply::json(&response))
}
// endregion: Utils

// region:    Test
#[cfg(test)]
#[path = "../_tests/web_grocery.rs"]
mod tests;
// endregion: Test
