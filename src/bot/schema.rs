use teloxide::prelude::*;

use crate::bot::Schema;

use super::{callback, command::Command, message};

pub fn root() -> Schema {
    let callbacks = callback_handler();
    let commands = command_handler();
    let messages = text_handler();
    let contact = contact_handler();
    dptree::entry().branch(callbacks).branch(
        Update::filter_message()
            .branch(commands)
            .branch(messages)
            .branch(contact),
    )
}

fn callback_handler() -> Schema {
    Update::filter_callback_query().endpoint(callback::callback)
}

fn text_handler() -> Schema {
    Message::filter_text().endpoint(message::text)
}

fn command_handler() -> Schema {
    dptree::entry()
        .filter_command::<Command>()
        .endpoint(message::command)
}

fn contact_handler() -> Schema {
    Message::filter_contact().endpoint(message::contact)
}
