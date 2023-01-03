use teloxide::types::{ButtonRequest, KeyboardButton, KeyboardMarkup, KeyboardRemove};

pub fn contact() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![vec![
        KeyboardButton::new("Share contact").request(ButtonRequest::Contact)
    ]])
    .one_time_keyboard(true)
    .input_field_placeholder("nothing to write here...".to_owned())
}

pub fn remove() -> KeyboardRemove {
    KeyboardRemove::new()
}
