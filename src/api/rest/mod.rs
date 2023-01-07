use warp::{Filter, Rejection, Reply};

use crate::storage::Database;

mod filters;
mod handlers;

/// Root filter.
pub fn schema(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    filters::users(db)
}
