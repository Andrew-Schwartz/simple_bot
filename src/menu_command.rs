use std::sync::Arc;

use command_data_derive::MenuCommand;
use discorsd::BotState;
use discorsd::commands::{InteractionUse, MenuCommand, Unused, Used};
use discorsd::errors::BotError;
use discorsd::http::channel::embed;
use discorsd::model::channel::ChannelMarkupExt;
use discorsd::model::components::ComponentId;
use discorsd::model::ids::ChannelId;

use crate::MyBot;

#[derive(MenuCommand)]
pub enum MyChoices {
    Avalon,
    Coup,
    Hangman,
    #[menu(label = "Exploding Kittens")]
    ExplodingKittens,
}

#[derive(Clone, Debug)]
pub struct MyStringMenu;

#[discorsd::async_trait]
impl MenuCommand for MyStringMenu {
    type Bot = MyBot;
    type Data = MyChoices;

    async fn run(
        &self,
        state: Arc<BotState<Self::Bot>>,
        interaction: InteractionUse<ComponentId, Unused>,
        data: Vec<Self::Data>,
    ) -> Result<InteractionUse<ComponentId, Used>, BotError> {
        let str: String = data.into_iter()
            .map(|d| d.to_string())
            .collect();
        interaction.respond(&state, str).await.map_err(|e| e.into())
    }
}

#[derive(Clone, Debug)]
pub struct MyChannelMenu;

#[discorsd::async_trait]
impl MenuCommand for MyChannelMenu {
    type Bot = MyBot;
    type Data = ChannelId;

    async fn run(
        &self,
        state: Arc<BotState<Self::Bot>>,
        interaction: InteractionUse<ComponentId, Unused>,
        data: Vec<Self::Data>,
    ) -> Result<InteractionUse<ComponentId, Used>, BotError> {
        interaction.respond(state, embed(|e| {
            e.fields(
                data.into_iter()
                    .enumerate()
                    .map(|(i, id)| (format!("Channel {i}"), id.mention()))
            )
        })).await.map_err(|e| e.into())
    }
}