use super::db::Db;
use crate::{model, security::UserCtx};
use serde::{Deserialize, Serialize};
use sqlb::HasFields;

// region:    Grocery Types
/// Grocery
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Grocery {
    pub id: i64,
    pub cid: i64, // creator id
    pub cost: i64,
    pub name: String,
    pub status: GroceryStatus,
}

/// Grocery Patch
#[derive(sqlb::Fields, Default, Debug, Clone, Deserialize)]
pub struct GroceryPatch {
    pub cost: Option<i64>,
    pub name: Option<String>,
    pub status: Option<GroceryStatus>,
}

/// Grocery Status
#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "grocery_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum GroceryStatus {
    Shelf,
    Basket,
}
sqlb::bindable!(GroceryStatus);
// endregion: Grocery Types

// region:    GroceryMac
/// Grocery Model Access Controller
pub struct GroceryMac;

impl GroceryMac {
    const TABLE: &'static str = "groceries";
    const COLUMNS: &'static [&'static str] = &["id", "cid", "cost", "name", "status"];
}

impl GroceryMac {
    /// create - create a new grocery
    pub async fn create(
        db: &Db,
        utx: &UserCtx,
        data: GroceryPatch,
    ) -> Result<Grocery, model::Error> {
        let mut fields = data.fields();
        fields.push(("cid", 123).into());
        let sb = sqlb::insert()
            .table(Self::TABLE)
            .data(fields)
            .returning(Self::COLUMNS);

        let grocery = sb.fetch_one(db).await?;

        Ok(grocery)
    }

    /// get - get a grocery
    pub async fn get(db: &Db, _utx: &UserCtx, id: i64) -> Result<Grocery, model::Error> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .and_where_eq("id", id);

        let result = sb.fetch_one(db).await;

        handle_fetch_one_result(result, Self::TABLE, id)
    }

    /// list - get list of groceries
    pub async fn list(db: &Db, _utx: &UserCtx) -> Result<Vec<Grocery>, model::Error> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .order_by("!id");

        // execute the query
        let grocery = sb.fetch_all(db).await?;

        Ok(grocery)
    }

    /// update - update a grocery
    pub async fn update(
        db: &Db,
        _utx: &UserCtx,
        id: i64,
        data: GroceryPatch,
    ) -> Result<Grocery, model::Error> {
        let sb = sqlb::update()
            .table(Self::TABLE)
            .data(data.fields())
            .and_where_eq("id", id)
            .returning(Self::COLUMNS);

        let result = sb.fetch_one(db).await;

        handle_fetch_one_result(result, Self::TABLE, id)
    }

    /// delete - delete a grocery
    pub async fn delete(db: &Db, _utx: &UserCtx, id: i64) -> Result<Grocery, model::Error> {
        let sb = sqlb::delete()
            .table(Self::TABLE)
            .returning(Self::COLUMNS)
            .and_where_eq("id", id);

        let result = sb.fetch_one(db).await;

        handle_fetch_one_result(result, Self::TABLE, id)
    }
}
// endregion: GroceryMac

// region:    Utils
/// handle_fetch_one_result - handle sqlx::Error
fn handle_fetch_one_result(
    result: Result<Grocery, sqlx::Error>,
    typ: &'static str,
    id: i64,
) -> Result<Grocery, model::Error> {
    result.map_err(|sqlx_error| match sqlx_error {
        sqlx::Error::RowNotFound => model::Error::EntityNotFound(typ, id.to_string()),
        other => model::Error::Sqlx(other),
    })
}
// endregion: Utils

#[cfg(test)]
#[path = "../_tests/model_grocery.rs"]
mod tests;
