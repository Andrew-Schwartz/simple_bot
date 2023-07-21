use std::borrow::Cow;
use std::fmt::Debug;
use std::sync::Arc;

use discorsd::{async_trait, BotState};
use discorsd::commands::{ButtonCommand, ButtonPressData, InteractionUse, SlashCommand, SlashCommandData, Unused, Used};
use discorsd::errors::BotError;
use discorsd::model::interaction_response::message;

use crate::MyBot;

#[derive(Debug, Clone)]
pub struct TestCommand;

#[discorsd::async_trait]
impl SlashCommand for TestCommand {
    type Bot = MyBot;
    type Data = ();
    type Use = Used;
    const NAME: &'static str = "test";

    fn description(&self) -> Cow<'static, str> {
        "this is a simple test command".into()
    }

    async fn run(
        &self,
        state: Arc<BotState<<Self as SlashCommand>::Bot>>,
        interaction: InteractionUse<SlashCommandData, Unused>,
        _data: Self::Data,
    ) -> Result<InteractionUse<SlashCommandData, Self::Use>, BotError> {
        interaction.respond(&state, message(|m| {
            m.content("Response!");
            m.embed(|e| {
                e.title("Componenets??");
            });
            m.button(&state, MyButton)
        })).await.map_err(|e| e.into())
    }
}

#[derive(Clone, Debug)]
struct MyButton;

#[async_trait]
impl ButtonCommand for MyButton {
    type Bot = MyBot;

    fn label(&self) -> String {
        "Hi".into()
    }

    async fn run(
        &self,
        state: Arc<BotState<Self::Bot>>,
        interaction: InteractionUse<ButtonPressData, Unused>
    ) -> Result<InteractionUse<ButtonPressData, Used>, BotError> {
        interaction.respond(state, "Button Pressed!")
            .await
            .map_err(|e| e.into())
    }
}