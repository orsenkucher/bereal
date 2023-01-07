use teloxide::{dispatching::HandlerExt, prelude::*};

use super::command::Command;
use super::Schema;
use super::{callback, command, message};

pub fn callback() -> Schema {
    Update::filter_callback_query().endpoint(callback::callback)
}

pub fn message() -> Schema {
    Update::filter_message().chain(Message::filter_text().endpoint(message::text))
}

pub fn command() -> Schema {
    use dptree::case;

    Update::filter_message()
        .filter_command::<Command>()
        .branch(case![Command::Help].endpoint(command::help))
        .branch(case![Command::AddFriends].endpoint(command::add_friends))
}

pub fn contact() -> Schema {
    Update::filter_message().chain(Message::filter_contact().endpoint(message::contact))
}
