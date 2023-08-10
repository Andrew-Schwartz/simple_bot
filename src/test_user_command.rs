use std::fmt::Debug;
use std::sync::Arc;

use discorsd::BotState;
use discorsd::commands::{InteractionUse, UserCommand, AppCommandData, Unused, Used};
use discorsd::errors::BotError;
use discorsd::model::interaction::PartialGuildMember;
use discorsd::model::user::User;

use crate::MyBot;

#[derive(Debug, Clone)]
pub struct TestUserCommand;

#[discorsd::async_trait]
impl UserCommand for TestUserCommand {
    type Bot = MyBot;

    fn name(&self) -> &'static str {
        "Test User"
    }

    async fn run(
        &self,
        state: Arc<BotState<<Self as UserCommand>::Bot>>,
        interaction: InteractionUse<AppCommandData, Unused>,
        target: User,
        guild_member: Option<PartialGuildMember>
    ) -> Result<InteractionUse<AppCommandData, Used>, BotError> {
        let guild_message: &str;
        // todo fix string quotation formatting
        if let Some(_gm) = guild_member {
            guild_message = " has guild info";
        }
        else {
            guild_message = "";
        }
        interaction.respond(state, format!("User {:?}{:?}", target.username, guild_message))
            .await
            .map_err(|e| e.into())
    }
}