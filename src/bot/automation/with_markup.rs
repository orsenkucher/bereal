use teloxide::payloads::{
    EditMessageText, EditMessageTextSetters, SendMessage, SendMessageSetters,
};
use teloxide::requests::Requester;
use teloxide::types::{Chat, MessageId, ReplyMarkup};

use crate::bot::Bot;

use super::message_request::{MessageRequest, Request};

#[derive(Debug, Clone)]
pub struct MessageWithMarkup<'a> {
    bot: &'a Bot,
    chat: &'a Chat,
    text: String,
    markup: ReplyMarkup,
}

impl<'a> MessageWithMarkup<'a> {
    pub fn new(bot: &'a Bot, chat: &'a Chat, text: String, markup: ReplyMarkup) -> Self {
        Self {
            bot,
            chat,
            text,
            markup,
        }
    }
}

impl MessageRequest for MessageWithMarkup<'_> {
    fn send(self) -> Request<SendMessage> {
        self.bot
            .send_message(self.chat.id, self.text)
            .reply_markup(self.markup)
    }

    fn edit(self, msg_id: MessageId) -> Option<Request<EditMessageText>> {
        if let ReplyMarkup::InlineKeyboard(markup) = self.markup {
            let req = self
                .bot
                .edit_message_text(self.chat.id, msg_id, self.text)
                .reply_markup(markup);
            Some(req)
        } else {
            None
        }
    }
}
