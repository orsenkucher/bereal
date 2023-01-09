use warp::{Filter, Rejection, Reply};

use crate::Database;

use super::handler;
use super::models::{ListOptions, NewUser};

/// All user filters combined.
pub fn users(db: Database) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    users_list(db.clone()).or(users_create(db.clone()))
}

/// GET /users?offset=3&limit=5
pub fn users_list(db: Database) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("users")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_db(db))
        .and_then(handler::list_users)
}

/// POST /users with JSON body
pub fn users_create(
    db: Database,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("users")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handler::create_user)
}

fn with_db(
    db: Database,
) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (NewUser,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
