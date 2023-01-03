use teloxide::prelude::*;
use teloxide::{adaptors::DefaultParseMode, types::ParseMode};

use crate::storage::Database;
use crate::BoxError;

mod callback;
mod command;
mod keyboard;
mod message;
pub mod schema;

pub type Bot = DefaultParseMode<teloxide::Bot>;

type HandlerResult = Result<(), BoxError>;

type Schema = dptree::Handler<
    'static,
    DependencyMap,
    HandlerResult,
    teloxide::dispatching::DpHandlerDescription,
>;

pub fn bot_from_env() -> Bot {
    teloxide::Bot::from_env().parse_mode(ParseMode::Html)
}

pub async fn dispatch(bot: Bot, schema: Schema, db: Database) {
    Dispatcher::builder(bot, schema)
        .dependencies(dptree::deps![db])
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
