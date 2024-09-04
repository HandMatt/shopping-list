use super::db::Db;
use crate::{model, security::UserCtx};
use serde::{Deserialize, Serialize};
use sqlb::HasFields;

// region:    Grocery Types
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Grocery {
    pub id: i64,
    pub cid: i64, // creator id
    pub cost: i64,
    pub name: String,
    pub status: GroceryStatus,
}

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
pub struct GroceryMac;

impl GroceryMac {
    const TABLE: &'static str = "groceries";
    const COLUMNS: &'static [&'static str] = &["id", "cid", "cost", "name", "status"];
}

// Grocery Model access Controller
impl GroceryMac {
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
}
// endregion: GroceryMac

#[cfg(test)]
#[path = "../_tests/model_grocery.rs"]
mod tests;
