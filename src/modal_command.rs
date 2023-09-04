use std::fmt::{Display, Formatter};
use std::sync::Arc;
use command_data_derive::Modal;

use discorsd::{async_trait, BotState};
use discorsd::commands::modal_command::*;
use discorsd::model::components::ComponentId;
use discorsd::model::message::TextMarkup;

use crate::{MyBot, BotError};

#[derive(Clone, Debug)]
pub struct MyModal;

#[derive(Modal)]
pub struct NewUser {
    favorite_color: String,
    favorite_number: i64,
    #[modal(long)]
    bio: String,
    #[modal(long)]
    feedback: Option<String>,
    least_fav_number: Option<i64>,
}

impl Display for NewUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "color: {}\nnumber: {}\n{}",
               self.favorite_color.code_inline(),
               self.favorite_number.code_inline(),
               self.bio.code_block(""),
        )?;
        if let Some(feedback) = &self.feedback {
            write!(f, "feedback: {}", feedback.code_block(""))?;
        }
        if let Some(least_fav_number) = &self.least_fav_number {
            write!(f, ":( num: {}", least_fav_number.code_inline())?;
        }
        Ok(())
    }
}

#[async_trait]
impl ModalCommand for MyModal {
    type Bot = MyBot;
    type Values = NewUser;

    async fn run(
        &self,
        state: Arc<BotState<MyBot>>,
        interaction: InteractionUse<ComponentId, Unused>,
        values: Self::Values,
    ) -> Result<InteractionUse<ComponentId, Used>, BotError> {
        // let values = values.into_iter().map(|s| s.code_block("rs")).collect::<String>();
        interaction.respond(state, format!("Modal Submitted! Values:\n{values}"))
            .await
            .map_err(Into::into)
    }
}
