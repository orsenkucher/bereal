use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

use super::{
    action::{MenuAction, NameAction},
    state::share_contact::UserState,
};

#[derive(Clone, Serialize, Deserialize, EnumIter, EnumString, Display, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum Language {
    EN,
    UA,
}

pub fn language(greet: &str) -> String {
    format!("{greet} Let's start! What's your language?")
}

pub fn language_resend() -> String {
    "Please, select your language. /resend".into()
}

pub fn language_keyboard(lang: &Language) -> String {
    match lang {
        Language::EN => "EN 🇬🇧",
        Language::UA => "UA 🇺🇦",
    }
    .into()
}

pub fn receive_name(lang: &Language) -> String {
    match lang {
        Language::EN => "What's your name?",
        Language::UA => "Як тебе звати?",
    }
    .into()
}

pub fn receive_name_resend(lang: &Language) -> String {
    match lang {
        Language::EN => "Let's get acquainted. /resend",
        Language::UA => "Давай знайомитись. /resend",
    }
    .into()
}

pub fn confirm_name(lang: &Language, old_name: Option<&str>, name: &str) -> String {
    let greet = match lang {
        Language::EN => "Hello",
        Language::UA => "Привіт",
    };
    match old_name {
        Some(old_name) => format!("{}, <s>{}</s>{}", greet, old_name, name),
        None => format!("{}, {}", greet, name),
    }
}

pub fn confirm_name_keyboard(kind: &NameAction) -> String {
    match kind {
        NameAction::Ok => "Ok",
    }
    .into()
}

pub fn contact(lang: &Language) -> String {
    match lang {
        Language::EN => "Share contact",
        _ => "Поділитись контактом",
    }
    .into()
}

pub fn contact_keyboard(lang: &Language) -> String {
    match lang {
        Language::EN => "Share contact",
        _ => "Поділитись контактом",
    }
    .into()
}

pub fn contact_placeholder(lang: &Language) -> String {
    match lang {
        Language::EN => "nothing to write here...",
        _ => "тут нічого писати не треба...",
    }
    .into()
}

pub fn contact_resend(lang: &Language) -> String {
    match lang {
        Language::EN => "Please share your contact. /resend",
        _ => "Будь ласка, поділіться контактом. /resend",
    }
    .into()
}

pub fn menu(user: UserState) -> String {
    match user.lang {
        Language::EN => format!("Hello, {}! You're in menu", user.name),
        Language::UA => format!("Привіт, {}! Ти в меню", user.name),
    }
}

pub fn menu_resend(lang: &Language) -> String {
    match lang {
        Language::EN => "To resend menu message use /resend",
        Language::UA => "Продублювати меню: /resend",
    }
    .into()
}

pub fn menu_keyboard(lang: &Language, kind: &MenuAction) -> String {
    use Language::*;
    use MenuAction::*;
    match (lang, kind) {
        (EN, Option1) => "Attestation",
        (EN, Option2) => "Materials",
        (UA, Option1) => "Атестація",
        (UA, Option2) => "Матеріали",
    }
    .into()
}
