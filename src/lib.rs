use std::error::Error;

use bot::MyBot;
use diesel::prelude::*;
use teloxide::prelude::*;

use self::models::{NewPost, Post};

pub mod bot;
pub mod migrations;
pub mod models;
pub mod schema;
pub mod storage;
pub mod util;

type BoxError = Box<dyn Error + Send + Sync + 'static>;
type HandlerResult = Result<(), BoxError>;

type MyHandler = dptree::Handler<
    'static,
    DependencyMap,
    HandlerResult,
    teloxide::dispatching::DpHandlerDescription,
>;

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("error saving new post")
}

pub async fn dispatch(bot: MyBot, schema: MyHandler) {
    Dispatcher::builder(bot, schema)
        .dependencies(dptree::deps![])
        .default_handler(|upd| async move {
            tracing::warn!("unhandled update: {upd:?}");
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "an error has occured in the dispatcher",
        ))
        .build()
        .dispatch()
        .await;
}

#[cfg(test)]
mod tests {
    struct MyType;

    fn is_normal<T: Sized + Send + Sync + Unpin>() {}

    #[test]
    fn normal_types() {
        is_normal::<MyType>();
    }
}
