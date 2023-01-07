use warp::{filters::BoxedFilter, http::Response, Filter, Reply};

use crate::graphql::{schema, Context};
use crate::Database;

mod filter;
mod handler;

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
        .or(filter::users(db))
        .with(warp::trace::request());

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await
}

fn homepage() -> BoxedFilter<(impl Reply,)> {
    warp::path::end()
        .map(|| {
            Response::builder()
                .header("content-type", "text/html")
                .body(format!(
                "<html><h1>BeReal API</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
            ))
        })
        .boxed()
}
