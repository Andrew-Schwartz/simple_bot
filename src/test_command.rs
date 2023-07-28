use std::borrow::Cow;
use std::fmt::Debug;
use std::sync::Arc;

use command_data_derive::{CommandData, CommandDataChoices};
use discorsd::BotState;
use discorsd::commands::{InteractionUse, SlashCommand, SlashCommandData, Unused, Used};
use discorsd::errors::BotError;
use discorsd::model::channel::ChannelType;
use discorsd::model::interaction_response::message;

use crate::echo_button::EchoButton;
use crate::menu_command::{MyUserMenu, MyStringMenu};
use crate::MyBot;

#[derive(CommandDataChoices)]
pub enum ComponentType {
    Button,
    StringMenu,
    UserMenu,
}

#[derive(CommandData)]
pub struct Data {
    component: ComponentType,
}

#[derive(Debug, Clone)]
pub struct TestCommand;

#[discorsd::async_trait]
impl SlashCommand for TestCommand {
    type Bot = MyBot;
    type Data = Data;
    type Use = Used;
    const NAME: &'static str = "test";

    fn description(&self) -> Cow<'static, str> {
        "this is a simple test command".into()
    }

    async fn run(
        &self,
        state: Arc<BotState<<Self as SlashCommand>::Bot>>,
        interaction: InteractionUse<SlashCommandData, Unused>,
        data: Self::Data,
    ) -> Result<InteractionUse<SlashCommandData, Self::Use>, BotError> {
        match data.component {
            ComponentType::Button => interaction.respond(&state, message(|m| {
                m.content("Response!");
                m.embed(|e| {
                    e.title("Buddon??");
                });
                m.button(&state, EchoButton, |b| b.label("Echo!!"));
            })).await.map_err(|e| e.into()),
            ComponentType::StringMenu => interaction.respond(&state, message(|m| {
                m.content("Response!");
                m.embed(|e| {
                    e.title("🧵 Menyu??");
                });
                m.menu(&state, MyStringMenu, |m| { m.max_values(2) });
            })).await.map_err(|e| e.into()),
            ComponentType::UserMenu => interaction.respond(&state, message(|m| {
                m.content("Response!");
                m.embed(|e| {
                    e.title("Yusr Menyu??");
                });
                m.menu(&state, MyUserMenu, |m| {
                    m.max_values(4);
                    m.placeholder("User!!!!")
                });
            })).await.map_err(|e| e.into()),
        }
    }
}