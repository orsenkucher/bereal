use teloxide::{prelude::*, types::Chat};

use crate::{models::NewUser, storage::Database};

use super::{keyboard, Bot, HandlerResult};

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

pub async fn text(bot: Bot, msg: Message, db: Database) -> HandlerResult {
    let greet = greet(&msg.chat);
    let user = db.user_by_chat_id(&msg.chat.id.to_string());
    let is_reg = matches!(user, Ok(u) if u.is_registered());

    if is_reg {
        let greet = format!("{}\nYou are already registered!", greet);
        bot.send_message(msg.chat.id, greet).await?;
    } else {
        let greet = format!("{}\nPlease share your contact with us", greet);
        bot.send_message(msg.chat.id, greet)
            .reply_markup(keyboard::contact())
            .await?;
    }
    Ok(())
}

fn greet(chat: &Chat) -> String {
    let name = chat
        .first_name()
        .map(|name| format!(", {}", name))
        .unwrap_or_default();
    format!("Hi{}👋", name)
}
