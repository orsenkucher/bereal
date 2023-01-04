use teloxide::prelude::*;
use teloxide::{types::Message, utils::command::BotCommands};

use super::{Bot, HandlerResult};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this message.")]
    Help,
    #[command(description = "share your friend contacts with bereal")]
    AddFriends,
}

pub async fn help(bot: Bot, msg: Message) -> HandlerResult {
    tracing::info!("help command for {}", msg.chat.id);
    let text = Command::descriptions().to_string();
    bot.send_message(msg.chat.id, text).await?;
    Ok(())
}

pub async fn add_friends(bot: Bot, msg: Message) -> HandlerResult {
    tracing::info!("add_friends command for {}", msg.chat.id);
    bot.send_message(msg.chat.id, "Please, share your friends")
        .await?;
    Ok(())
}
