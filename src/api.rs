use anyhow::Error;
use serde::{Deserialize, Serialize};
use warp::hyper::StatusCode;
use warp::{filters::BoxedFilter, Filter, Reply};
use warp::{http, reply};

use crate::graphql::{schema, Context};
use crate::Database;

mod filter;
mod handler;
mod models;

macro_rules! warp_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => Ok(val.into_response()),
            Err(err) => {
                return Ok(err.into_response());
            }
        }
    };
}

pub(crate) use warp_try;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ErrorReply {
    message: String,
    code: isize,
}

impl ErrorReply {
    fn database_error(message: String) -> Self {
        Self { code: 1, message }
    }
}

enum ReplyError {
    DatabaseError(Error),
}

impl Reply for ReplyError {
    fn into_response(self) -> reply::Response {
        let error_reply = match self {
            ReplyError::DatabaseError(err) => ErrorReply::database_error(err.to_string()),
        };
        warp::reply::with_status(
            warp::reply::json(&error_reply),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response()
    }
}

/// Creates GrpahQL and REST API layer.
/// Takes database service.
pub async fn run(db: Database) {
    tracing::info!("listening on 127.0.0.1:8080");

    let db_clone = db.clone();
    let state = warp::any().map(move || Context::new(db_clone.clone()));
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());

    let routes = warp::get()
        .and(warp::path("graphiql"))
        .and(juniper_warp::graphiql_filter("/graphql", None))
        .or(homepage())
        .or(warp::path("graphql").and(graphql_filter))
        .or(filter::users(db.clone()))
        .or(filter::friends(db))
        .with(warp::trace::request());

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await
}

fn homepage() -> BoxedFilter<(impl Reply,)> {
    warp::path::end()
        .map(|| {
            http::Response::builder()
                .header("content-type", "text/html")
                .body(format!(
                "<html><h1>BeReal API</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
            ))
        })
        .boxed()
}
