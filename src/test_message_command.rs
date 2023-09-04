use std::fmt::Debug;
use std::sync::Arc;

use discorsd::BotState;
use discorsd::commands::{InteractionUse, MessageCommand, AppCommandData, Unused, Used};
use discorsd::errors::BotError;
use discorsd::model::message::Message;

use crate::MyBot;

#[derive(Debug, Clone)]
pub struct TestMessageCommand;

#[discorsd::async_trait]
impl MessageCommand for TestMessageCommand {
    type Bot = MyBot;

    fn name(&self) -> &'static str {
        "Test Message"
    }

    async fn run(
        &self,
        state: Arc<BotState<<Self as MessageCommand>::Bot>>,
        interaction: InteractionUse<AppCommandData, Unused>,
        target: Message
    ) -> Result<InteractionUse<AppCommandData, Used>, BotError> {
        interaction.respond(state, format!("Message by {:?}", target.author.username))
            .await
            .map_err(Into::into)
    }
}