use std::sync::Arc;

use command_data_derive::MenuCommand;
use discorsd::BotState;
use discorsd::commands::{InteractionUse, MenuCommand, Unused, Used};
use discorsd::errors::BotError;
use discorsd::model::user::UserMarkup;
use discorsd::model::ids::UserId;
use discorsd::model::interaction::MenuSelectData;

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
        interaction: InteractionUse<MenuSelectData, Unused>,
        data: Vec<Self::Data>,
    ) -> Result<InteractionUse<MenuSelectData, Used>, BotError> {
        println!("interaction.data.resolved = {:#?}", interaction.data.resolved);
        let str: String = data.into_iter()
            .map(|d| d.to_string())
            .collect();
        interaction.respond(&state, str).await.map_err(Into::into)
    }
}

#[derive(Clone, Debug)]
pub struct MyUserMenu;

#[discorsd::async_trait]
impl MenuCommand for MyUserMenu {
    type Bot = MyBot;
    type Data = UserId;

    async fn run(
        &self,
        state: Arc<BotState<Self::Bot>>,
        interaction: InteractionUse<MenuSelectData, Unused>,
        data: Vec<Self::Data>,
    ) -> Result<InteractionUse<MenuSelectData, Used>, BotError> {
        println!("interaction.data.resolved = {:#?}", interaction.data.resolved);
        let message: String = data.into_iter()
            .enumerate()
            .map(|(i, id)| format!("Channel {i}: {}\n", id.ping()))
            .collect();
        interaction.respond(state, message).await.map_err(Into::into)
    }
}