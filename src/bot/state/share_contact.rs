use serde::{Deserialize, Serialize};
use teloxide::types::{Contact, ReplyMarkup};

use crate::bot::keyboard;
use crate::bot::language::{self, Language};

use super::{StateKeyboard, StateMessage, StateResend};

/// Information gathered about the user in onboarding process.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserState {
    pub lang: Language,
    pub name: String,
    pub contact: Option<Contact>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShareContact {
    pub user: UserState,
}

impl ShareContact {
    pub fn new(user: UserState) -> Self {
        Self { user }
    }
}

impl StateMessage for ShareContact {
    type Payload = ();

    fn message_text(&self, _payload: Self::Payload) -> String {
        language::contact(&self.user.lang)
    }
}

impl StateKeyboard for ShareContact {
    type Payload = ();

    fn message_keyboard(&self, _payload: Self::Payload) -> ReplyMarkup {
        keyboard::contact(&self.user.lang).into()
    }
}

impl StateResend for ShareContact {
    fn resend_text(&self) -> String {
        language::contact_resend(&self.user.lang)
    }
}
