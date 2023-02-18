use teloxide::dispatching::dialogue::{self, InMemStorage};
use teloxide::prelude::*;
use teloxide::{adaptors::DefaultParseMode, types::ParseMode};

use crate::Database;

use self::state::State;

mod action;
mod automation;
mod callback;
mod command;
mod handler;
mod keyboard;
mod language;
mod message;
mod state;

pub type Bot = DefaultParseMode<teloxide::Bot>;
pub type Dialogue = dialogue::Dialogue<State, InMemStorage<State>>;

type HandlerResult = anyhow::Result<()>;

type Schema = dptree::Handler<
    'static,
    DependencyMap,
    HandlerResult,
    teloxide::dispatching::DpHandlerDescription,
>;

pub fn bot_from_env() -> Bot {
    teloxide::Bot::from_env().parse_mode(ParseMode::Html)
}

pub fn schema() -> Schema {
    dptree::entry()
        .branch(handler::callback())
        .branch(handler::command())
        .branch(handler::message())
        .branch(handler::contact())
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
