use std::str::FromStr;

use anyhow::bail;
use teloxide::{prelude::*, types::Chat};

use crate::models::NewUser;
use crate::Database;

use super::{automation::Automation, keyboard, state::State, Bot, Dialogue, HandlerResult};

pub async fn contact(bot: Bot, msg: Message, db: Database) -> HandlerResult {
    if let Some(contact) = msg.contact() {
        tracing::debug!("contact message: {:?}", contact);
        let chat_id = msg.chat.id.to_string();
        let new_user = NewUser::joined_now(&contact.phone_number, &chat_id);
        let user = db.create_user(new_user)?;
        tracing::debug!("created user: {user:?}");
        bot.send_message(msg.chat.id, "Thanks!")
            .reply_markup(keyboard::remove())
            .await?;
    }
    Ok(())
}

pub async fn start(
    bot: Bot,
    msg: Message,
    db: Database,
    state: State,
    dialogue: Dialogue,
) -> HandlerResult {
    use super::state::{Language, Menu};

    let State::Start = state else {
        bail!("Bad state: {:?}", state);
    };

    let greet = greet(&msg.chat);
    let user = db.user_by_chat_id(&msg.chat.id.to_string());
    let user = user.as_ref().map_or(None, |u| u.as_registered());
    if let Some(user) = user {
        let lang = FromStr::from_str(&user.language)?;
        let mut auto = Automation::new((greet, user), Menu::new(lang));
        auto.send_message(&bot, &msg.chat).await?;
        dialogue.update(State::Menu(auto)).await?;
    } else {
        let mut auto = Automation::new((greet,), Language);
        auto.send_message(&bot, &msg.chat).await?;
        dialogue.update(State::Language(auto)).await?;
    }

    Ok(())
}

pub async fn language(bot: Bot, msg: Message, db: Database) -> HandlerResult {
    Ok(())
}

fn greet(chat: &Chat) -> String {
    let name = chat
        .first_name()
        .map(|name| format!(", {}", name))
        .unwrap_or_default();
    format!("Hi{}👋!", name)
}
