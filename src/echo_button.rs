use std::sync::Arc;

use discorsd::{async_trait, BotState};
use discorsd::commands::{ButtonCommand, ButtonPressData, InteractionUse, Unused, Used};
use discorsd::errors::BotError;

use crate::MyBot;

pub async fn echo(
    state: Arc<BotState<MyBot>>,
    interaction: InteractionUse<ButtonPressData, Unused>,
) -> Result<InteractionUse<ButtonPressData, Used>, BotError> {
    interaction.respond(state, "Button Pressed!")
        .await
        .map_err(|e| e.into())
}

#[derive(Clone, Debug)]
pub struct EchoButton;

#[async_trait]
impl ButtonCommand for EchoButton {
    type Bot = MyBot;

    async fn run(
        &self,
        state: Arc<BotState<Self::Bot>>,
        interaction: InteractionUse<ButtonPressData, Unused>,
    ) -> Result<InteractionUse<ButtonPressData, Used>, BotError> {
        echo(state, interaction).await
    }
}
