use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(description = "These commands are supported:")]
pub enum Command {
    #[command(rename = "lowercase", description = "display this message.")]
    Help,
}
