use strum::IntoEnumIterator;
use teloxide::types::{
    ButtonRequest, InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup,
    KeyboardRemove,
};

use super::{
    action::{MenuAction, NameAction},
    language::{self, Language},
};

pub fn languages() -> InlineKeyboardMarkup {
    let langs = Language::iter().collect::<Vec<_>>();
    let langs: Vec<Vec<_>> = langs
        .chunks(2)
        .map(|ch| ch.iter().map(language_button).collect())
        .collect();
    InlineKeyboardMarkup::new(langs)
}

fn language_button(lang: &Language) -> InlineKeyboardButton {
    InlineKeyboardButton::callback(language::language_keyboard(lang), lang.to_string())
}

pub fn contact(lang: &Language) -> KeyboardMarkup {
    KeyboardMarkup::new(vec![vec![KeyboardButton::new(language::contact_keyboard(
        lang,
    ))
    .request(ButtonRequest::Contact)]])
    .one_time_keyboard(true)
    .input_field_placeholder(language::contact_placeholder(lang))
}

pub fn remove() -> KeyboardRemove {
    KeyboardRemove::new()
}

pub fn confirm_name() -> InlineKeyboardMarkup {
    let ok = NameAction::Ok;
    InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
        language::confirm_name_keyboard(&ok),
        ok.to_string(),
    )]])
}

pub fn menu(lang: &Language) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup::new(vec![MenuAction::iter()
        .map(|val| {
            InlineKeyboardButton::callback(language::menu_keyboard(lang, &val), val.to_string())
        })
        .collect::<Vec<_>>()])
}
