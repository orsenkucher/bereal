use teloxide::payloads::{EditMessageText, SendMessage};
use teloxide::requests::Requester;
use teloxide::types::{Chat, MessageId};

use crate::bot::Bot;

use super::message_request::{MessageRequest, Request};

#[derive(Debug, Clone)]
pub struct MessageWithoutMarkup<'a> {
    bot: &'a Bot,
    chat: &'a Chat,
    text: String,
}

impl<'a> MessageWithoutMarkup<'a> {
    pub fn new(bot: &'a Bot, chat: &'a Chat, text: String) -> Self {
        Self { bot, chat, text }
    }
}

impl MessageRequest for MessageWithoutMarkup<'_> {
    fn send(self) -> Request<SendMessage> {
        self.bot.send_message(self.chat.id, self.text)
    }

    fn edit(self, msg_id: MessageId) -> Option<Request<EditMessageText>> {
        Some(self.bot.edit_message_text(self.chat.id, msg_id, self.text))
    }
}
