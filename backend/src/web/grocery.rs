use super::{filter_auth::do_auth, filter_utils::with_db};
use crate::{
    model::{Db, GroceryMac},
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

    list
}

/// GET groceries/
async fn grocery_list(db: Arc<Db>, utx: UserCtx) -> Result<Json, warp::Rejection> {
    let groceries = GroceryMac::list(&db, &utx).await?;
    json_response(groceries)
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
