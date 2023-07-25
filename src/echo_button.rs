use std::sync::Arc;

use discorsd::{async_trait, BotState};
use discorsd::commands::{ButtonCommand, InteractionUse, Unused, Used};
use discorsd::errors::BotError;
use discorsd::model::new_interaction::ButtonPressData;

use crate::MyBot;

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
        interaction.respond(state, "Button Pressed!")
            .await
            .map_err(|e| e.into())
    }
}
