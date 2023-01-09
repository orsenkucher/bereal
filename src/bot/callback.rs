use teloxide::prelude::*;

use super::{Bot, HandlerResult};

pub async fn callback(bot: Bot, q: CallbackQuery) -> HandlerResult {
    tracing::debug!("callback query: {q:?}");
    if let Some(Message { chat, .. }) = q.message {
        bot.send_message(chat.id, "Callback received!").await?;
    }
    Ok(())
}
