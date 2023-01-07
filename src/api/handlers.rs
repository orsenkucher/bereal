use std::convert::Infallible;

use warp::{hyper::StatusCode, reply::Response, Reply};

use crate::{
    models::{ListOptions, User},
    storage::Database,
};

macro_rules! warp_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                return Ok(err.into_response());
            }
        }
    };
}

enum MyServerError {
    DatabaseInvalidConnection(anyhow::Error),
}

impl Reply for MyServerError {
    fn into_response(self) -> Response {
        match self {
            MyServerError::DatabaseInvalidConnection(err) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

pub async fn list_users(opts: ListOptions, db: Database) -> Result<impl Reply, Infallible> {
    let users = db.users().map_err(MyServerError::DatabaseInvalidConnection);
    let users = warp_try!(users);
    let users = users
        .into_iter()
        .skip(opts.offset.unwrap_or(0))
        .take(opts.limit.unwrap_or(usize::MAX))
        .collect::<Vec<_>>();

    Ok(warp::reply::json(&users).into_response())
}

pub async fn create_user(user: User, db: Database) -> Result<impl Reply, Infallible> {
    Ok(warp::reply().into_response())
}