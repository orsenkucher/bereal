use std::error::Error;

use bot::MyBot;
use teloxide::prelude::*;

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
