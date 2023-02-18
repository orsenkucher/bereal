use serde::{Deserialize, Serialize};
use teloxide::types::ReplyMarkup;

use crate::bot::{keyboard, language};

use super::{StateKeyboard, StateMessage, StateResend};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Language;

pub type Payload = (String,);

impl StateMessage for Language {
    type Payload = Payload;

    fn message_text(&self, payload: Self::Payload) -> String {
        let greet = payload.0;
        language::language(&greet)
    }
}

impl StateKeyboard for Language {
    type Payload = Payload;

    fn message_keyboard(&self, _payload: Self::Payload) -> ReplyMarkup {
        keyboard::languages().into()
    }
}

impl StateResend for Language {
    fn resend_text(&self) -> String {
        language::language_resend()
    }
}
