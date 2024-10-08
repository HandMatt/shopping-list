use crate::model::Db;
use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;

/// with_db - warp filter for injecting db
pub fn with_db(db: Arc<Db>) -> impl Filter<Extract = (Arc<Db>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
