use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::{dispatching::HandlerExt, prelude::*};

use super::command::Command;
use super::state::State;
use super::Schema;
use super::{callback, command, message};

pub fn callback() -> Schema {
    use dptree::case;
    Update::filter_callback_query()
        .enter_dialogue::<CallbackQuery, InMemStorage<State>, State>()
        .branch(case![State::Language(auto)].endpoint(callback::language))
}

pub fn message() -> Schema {
    use dptree::case;
    Update::filter_message().chain(
        Message::filter_text()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(case![State::Start].endpoint(message::start))
            .branch(case![State::Language(auto)].endpoint(message::language)),
    )
}

pub fn command() -> Schema {
    use dptree::case;
    Update::filter_message()
        .filter_command::<Command>()
        .branch(case![Command::Help].endpoint(command::help))
        .branch(case![Command::AddFriends].endpoint(command::add_friends))
}

pub fn contact() -> Schema {
    use dptree::case;
    Update::filter_message().chain(
        Message::filter_contact()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(case![State::ShareContact(auto)].endpoint(message::contact)),
    )
}
