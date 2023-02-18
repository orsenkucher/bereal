use serde::{Deserialize, Serialize};
use teloxide::types::ReplyMarkup;

use crate::bot::language::Language;
use crate::bot::{keyboard, language};
use crate::models::RegisteredUser;

use super::{StateKeyboard, StateMessage, StateResend};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Menu {
    // pub user: UserState,
    pub lang: Language,
}

// greet, user
pub type Payload = (String, RegisteredUser);
// pub type Payload = (String, User);

impl Menu {
    // pub fn new(user: UserState) -> Self {
    //     Self { user }
    // }
    pub fn new(lang: Language) -> Self {
        Self { lang }
    }
}

impl StateMessage for Menu {
    type Payload = Payload;

    fn message_text(&self, payload: Self::Payload) -> String {
        let greet = payload.0;
        // what to do when menu is resent?
        // language::menu(user);
        format!("{}\nYou are already registered!", greet)
    }
}

impl StateKeyboard for Menu {
    type Payload = Payload;

    fn message_keyboard(&self, payload: Self::Payload) -> ReplyMarkup {
        keyboard::menu(&self.lang).into()
    }
}

impl StateResend for Menu {
    fn resend_text(&self) -> String {
        language::menu_resend(&self.lang)
    }
}
