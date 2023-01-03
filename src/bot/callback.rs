use teloxide::prelude::*;

use super::{Bot, HandlerResult};

pub async fn callback(bot: Bot, q: CallbackQuery) -> HandlerResult {
    Ok(())
}
