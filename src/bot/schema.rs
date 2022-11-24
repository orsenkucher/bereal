use teloxide::prelude::*;

use crate::MyHandler;

use super::{callback, command::Command, message};

pub fn root() -> MyHandler {
    let callbacks = callback_branch();
    let commands = command_branch();
    let messages = text_branch();
    dptree::entry()
        .branch(callbacks)
        .branch(Update::filter_message().branch(commands).branch(messages))
}

fn callback_branch() -> MyHandler {
    Update::filter_callback_query().endpoint(callback::callback)
}

fn text_branch() -> MyHandler {
    Message::filter_text().endpoint(message::text)
}

fn command_branch() -> MyHandler {
    dptree::entry()
        .filter_command::<Command>()
        .endpoint(message::command)
}
