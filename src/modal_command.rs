use std::sync::Arc;

use discorsd::{async_trait, BotState, modal_values};
use discorsd::commands::modal_command::*;
use discorsd::errors::BotError;
use discorsd::model::components::ComponentId;
use discorsd::model::message::TextMarkup;

use crate::MyBot;

#[derive(Clone, Debug)]
pub struct MyModal;

pub struct Data(String);

// todo is having this macro actually better?
modal_values!(Data => 1; vec; {
    Ok(Self(vec.remove(0)))
});

#[async_trait]
impl ModalCommand for MyModal {
    type Bot = MyBot;
    type Values = [String; 2];

    async fn run(
        &self,
        state: Arc<BotState<MyBot>>,
        interaction: InteractionUse<ComponentId, Unused>,
        values: Self::Values,
    ) -> Result<InteractionUse<ComponentId, Used>, BotError> {
        let values = values.into_iter().map(|s| s.code_block("rs")).collect::<String>();
        interaction.respond(state, format!("Modal Submitted! Values:\n{values}"))
            .await
            .map_err(|e| e.into())
    }
}
