use serde::{Deserialize, Serialize};
use teloxide::types::ReplyMarkup;

use crate::bot::{
    keyboard,
    language::{self, Language},
};

use super::{StateKeyboard, StateMessage};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfirmName {
    pub lang: Language,
    pub name: String,
}

// old name, new name
pub type Payload = (Option<String>, String);

impl ConfirmName {
    pub fn new(lang: Language, name: String) -> Self {
        Self { lang, name }
    }
}

impl StateMessage for ConfirmName {
    type Payload = Payload;

    fn message_text(&self, payload: Self::Payload) -> String {
        language::confirm_name(&self.lang, payload.0.as_deref(), payload.1.as_ref())
    }
}

impl StateKeyboard for ConfirmName {
    type Payload = Payload;

    fn message_keyboard(&self, _payload: Self::Payload) -> ReplyMarkup {
        keyboard::confirm_name().into()
    }
}
