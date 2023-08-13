use std::sync::Arc;

use discorsd::{async_trait, BotState};
use discorsd::commands::{ModalCommand, InteractionUse, Unused, Used};
use discorsd::errors::BotError;
use discorsd::model::interaction::ModalSubmitData;

use crate::MyBot;

#[derive(Clone, Debug)]
pub struct MyModal;

#[async_trait]
impl ModalCommand for MyModal {
    type Bot = MyBot;

    async fn run(
        &self,
        state: Arc<BotState<Self::Bot>>,
        interaction: InteractionUse<ModalSubmitData, Unused>,
    ) -> Result<InteractionUse<ModalSubmitData, Used>, BotError> {
        interaction.respond(state, "Modal Submitted!")
            .await
            .map_err(|e| e.into())
    }
}
