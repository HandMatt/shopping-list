mod db;
mod grocery;

// re-export
pub use db::init_db;
pub use db::Db;

// region:    Error
/// model error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
// endregion: Error
