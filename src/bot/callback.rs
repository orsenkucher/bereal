use anyhow::bail;
use teloxide::prelude::*;

use super::{state::State, Bot, HandlerResult};

pub async fn callback(bot: Bot, q: CallbackQuery) -> HandlerResult {
    tracing::debug!("callback query: {q:?}");
    if let Some(Message { chat, .. }) = q.message {
        bot.send_message(chat.id, "Callback received!").await?;
    }
    Ok(())
}

pub async fn language(bot: Bot, q: CallbackQuery, state: State) -> HandlerResult {
    tracing::debug!("callback::language");
    let State::Language(auto) = state else {
        bail!("Bad state: {:?}", state);
    };
    Ok(())
}
