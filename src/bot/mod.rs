use teloxide::prelude::*;
use teloxide::{adaptors::DefaultParseMode, types::ParseMode};

mod callback;
mod command;
mod message;
pub mod schema;

pub type MyBot = DefaultParseMode<Bot>;

pub fn bot_from_env() -> MyBot {
    Bot::from_env().parse_mode(ParseMode::Html)
}
