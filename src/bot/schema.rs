use teloxide::{dispatching::HandlerExt, prelude::*};

use crate::bot::Schema;

use super::{
    callback,
    command::{self, Command},
    message,
};

pub fn root() -> Schema {
    let callback = callback_handler();
    let command = command_handler();
    let message = message_handler();
    let contact = contact_handler();

    dptree::entry()
        .branch(callback)
        .branch(command)
        .branch(message)
        .branch(contact)
}

fn callback_handler() -> Schema {
    Update::filter_callback_query().endpoint(callback::callback)
}

fn message_handler() -> Schema {
    Update::filter_message().chain(Message::filter_text().endpoint(message::text))
}

fn command_handler() -> Schema {
    use dptree::case;

    Update::filter_message()
        .filter_command::<Command>()
        .branch(case![Command::Help].endpoint(command::help))
        .branch(case![Command::AddFriends].endpoint(command::add_friends))
}

fn contact_handler() -> Schema {
    Update::filter_message().chain(Message::filter_contact().endpoint(message::contact))
}
