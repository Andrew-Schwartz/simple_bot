use std::borrow::Cow;
use std::fmt::Debug;
use std::sync::Arc;

use command_data_derive::{CommandData, CommandDataChoices};
use discorsd::BotState;
use discorsd::commands::{AppCommandData, InteractionUse, ModalCommand, SlashCommand, Unused, Used};
use discorsd::model::interaction_response::{message, modal};

use crate::echo_button::EchoButton;
use crate::menu_command::{MyStringMenu, MyUserMenu};
use crate::modal_command::MyModal;
use crate::{MyBot, BotError};

#[derive(CommandDataChoices)]
pub enum ComponentType {
    Button,
    StringMenu,
    UserMenu,
    Modal,
}

#[derive(CommandData)]
pub struct Data {
    component: ComponentType,
}

#[derive(Debug, Clone)]
pub struct TestModalCommand;

#[discorsd::async_trait]
impl SlashCommand for TestModalCommand {
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
        interaction: InteractionUse<AppCommandData, Unused>,
        data: Self::Data,
    ) -> Result<InteractionUse<AppCommandData, Self::Use>, BotError> {
        match data.component {
            ComponentType::Button => interaction.respond(&state, message(|m| {
                m.content("Response!");
                m.embed(|e| {
                    e.title("Buddon??");
                });
                m.button(&state, EchoButton, |b| b.label("Echo!!"));
            })).await.map_err(Into::into),
            ComponentType::StringMenu => interaction.respond(&state, message(|m| {
                m.content("Response!");
                m.embed(|e| {
                    e.title("🧵 Menyu??");
                });
                m.menu(&state, MyStringMenu, |m| { m.max_values(2) });
            })).await.map_err(Into::into),
            ComponentType::UserMenu => interaction.respond(&state, message(|m| {
                m.content("Response!");
                m.embed(|e| {
                    e.title("Yusr Menyu??");
                });
                m.menu(&state, MyUserMenu, |m| {
                    m.max_values(4);
                    m.placeholder("User!!!!");
                });
            })).await.map_err(Into::into),
            ComponentType::Modal => interaction.respond_modal(
                &state,
                modal(
                    &state,
                    MyModal,
                    <MyModal as ModalCommand>::Values::builder(),
                )
            ).await.map_err(Into::into),
        }
    }
}