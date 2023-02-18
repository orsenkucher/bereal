use serde::{Deserialize, Serialize};
use teloxide::types::ReplyMarkup;

use crate::bot::automation::Automation;

pub(crate) mod confirm_name;
pub(crate) mod language;
pub(crate) mod menu;
pub(crate) mod receive_name;
pub(crate) mod share_contact;

pub use self::confirm_name::ConfirmName;
pub use self::language::Language;
pub use self::menu::Menu;
pub use self::receive_name::ReceiveName;
pub use self::share_contact::ShareContact;

pub trait StateMessage {
    /// Payload represents useful data provided on message creation.
    type Payload;

    fn message_text(&self, payload: Self::Payload) -> String;
}

pub trait StateKeyboard {
    /// Payload represents useful data provided on message keyboard creation.
    type Payload;

    fn message_keyboard(&self, payload: Self::Payload) -> ReplyMarkup;
}

pub trait StateResend {
    fn resend_text(&self) -> String;
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum State {
    #[default]
    Start,
    Language(Automation<Language, language::Payload>),
    ReceiveName(Automation<ReceiveName, receive_name::Payload>),
    ConfirmName(Automation<ConfirmName, confirm_name::Payload>),
    ShareContact(Automation<ShareContact>),
    Menu(Automation<Menu, menu::Payload>),
}
