use serde::de::DeserializeOwned;
use warp::{Filter, Rejection, Reply};

use crate::Database;

use super::handler;
use super::models::ListOptions;

/// All user filters combined.
pub fn users(db: Database) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    users_list(db.clone()).or(users_create(db))
}

pub fn friends(db: Database) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    friends_create(db)
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

pub fn friends_create(
    db: Database,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("friends")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handler::add_friend)
}

fn with_db(
    db: Database,
) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body<T>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone
where
    T: DeserializeOwned + Send,
{
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
