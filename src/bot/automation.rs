mod message_request;
mod with_markup;
mod without_markup;

use std::ops::Deref;

use serde::{Deserialize, Serialize};
use teloxide::prelude::*;
use teloxide::types::{Chat, MessageId};

use crate::bot::state::{StateKeyboard, StateMessage, StateResend};
use crate::bot::{Bot, HandlerResult};

use self::message_request::MessageRequest;
use self::with_markup::MessageWithMarkup;
use self::without_markup::MessageWithoutMarkup;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LastFlag(bool);

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Automation<T, P = ()> {
    last: LastFlag,
    msg_id: Option<MessageId>,
    target: T,
    payload: P,
}

impl<T, P> Deref for Automation<T, P> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.target
    }
}

impl<T, P> Automation<T, P>
where
    P: Clone,
{
    pub fn new(payload: P, target: T) -> Self {
        Self::builder(LastFlag(true), None)(payload, target)
    }

    pub fn builder(last: LastFlag, msg_id: Option<MessageId>) -> impl FnOnce(P, T) -> Self {
        move |payload, target| Self {
            last,
            msg_id,
            target,
            payload,
        }
    }

    pub fn set_payload(&mut self, payload: P) {
        self.payload = payload;
    }

    pub fn get_payload(&self) -> P {
        self.payload.clone()
    }

    pub fn set_msg_id(&mut self, msg_id: MessageId) {
        self.msg_id = Some(msg_id)
    }

    pub fn clear_msg_id(&mut self) {
        self.msg_id = None
    }

    pub fn get_msg_id(&self) -> Option<MessageId> {
        self.msg_id
    }

    pub fn next<U, Q>(self, payload: Q, map_target: impl FnOnce(T) -> U) -> Automation<U, Q> {
        Automation {
            target: map_target(self.target),
            last: self.last,
            msg_id: self.msg_id,
            payload,
        }
    }

    pub fn not_last(self) -> Self {
        Self {
            last: LastFlag(false),
            ..self
        }
    }
}

impl<T, P> Automation<T, P> {
    async fn send(
        &mut self,
        bot: &Bot,
        chat: &Chat,
        msg: impl MessageRequest + Clone,
    ) -> HandlerResult {
        if let Some(msg_id) = self.msg_id {
            let msg_id = match (self.last.0, msg.clone().edit(msg_id)) {
                (true, Some(edit)) => {
                    edit.await?;
                    msg_id
                }
                _ => {
                    bot.delete_message(chat.id, msg_id).await?;
                    let sent = msg.send().await?;
                    sent.id
                }
            };
            self.msg_id = Some(msg_id);
        } else {
            let sent = msg.send().await?;
            self.msg_id = Some(sent.id);
        }
        self.last = LastFlag(true);
        Ok(())
    }
}

impl<T, P> Automation<T, P>
where
    T: StateMessage<Payload = P>,
    P: Clone,
{
    pub async fn send_message_no_markup(&mut self, bot: &Bot, chat: &Chat) -> HandlerResult {
        let payload = self.get_payload();
        let text = self.message_text(payload);
        let msg = MessageWithoutMarkup::new(bot, chat, text);
        self.send(bot, chat, msg).await?;
        Ok(())
    }
}

impl<T, P> Automation<T, P>
where
    T: StateMessage<Payload = P> + StateKeyboard<Payload = P>,
    P: Clone,
{
    pub async fn send_message(&mut self, bot: &Bot, chat: &Chat) -> HandlerResult {
        let payload = self.get_payload();
        let text = self.message_text(payload.clone());
        let markup = self.message_keyboard(payload);
        let msg = MessageWithMarkup::new(bot, chat, text, markup);
        self.send(bot, chat, msg).await?;
        Ok(())
    }
}

impl<T, P> Automation<T, P>
where
    T: StateResend,
{
    pub async fn resend(&mut self, bot: &Bot, chat: &Chat) -> HandlerResult {
        let text = self.resend_text();
        bot.send_message(chat.id, text).await?;
        Ok(())
    }
}
