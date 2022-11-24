use teloxide::{prelude::*, types::Me};

use crate::HandlerResult;

pub async fn text(bot: Bot, msg: Message, me: Me) -> HandlerResult {
    bot.send_message(msg.chat.id, "Let's start!").await?;
    Ok(())
}

pub async fn command(bot: Bot, msg: Message, me: Me) -> HandlerResult {
    Ok(())
}
