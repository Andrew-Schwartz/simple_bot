#![warn(clippy::pedantic)]
#![allow(
    clippy::wildcard_imports
)]

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

use discorsd::{Bot, BotExt, BotState};
use discorsd::commands::{MessageCommand, SlashCommandRaw, UserCommand};
use discorsd::http::channel::{embed, MessageChannelExt};
use discorsd::model::guild::Guild;
use discorsd::model::message::Color;

use config::*;

use crate::test_message_command::TestMessageCommand;
use crate::test_modal_command::TestModalCommand;
use crate::test_user_command::TestUserCommand;

mod test_modal_command;
mod echo_button;
mod menu_command;
mod modal_command;
mod config;
mod test_user_command;
mod test_message_command;

pub struct MyBot;
pub struct MyError;

impl Debug for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyError")
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyError")
    }
}

impl Error for MyError {}

pub type BotError = discorsd::errors::BotError<MyError>;

#[discorsd::async_trait]
impl Bot for MyBot {
    type Error = MyError;

    fn token(&self) -> String {
        TOKEN.into()
    }

    fn global_commands() -> &'static [&'static dyn SlashCommandRaw<Bot=Self>] { &[&TestModalCommand] }

    fn global_user_commands() -> &'static [&'static dyn UserCommand<Bot=Self>] { &[&TestUserCommand] }

    fn global_message_commands() -> &'static [&'static dyn MessageCommand<Bot=Self>] { &[&TestMessageCommand] }

    async fn guild_create(&self, guild: Guild, state: Arc<BotState<Self>>) -> Result<(), BotError> {
        if guild.id != GUILD { return Ok(()); }
        guild.channels.get(CHANNEL).unwrap()
            .send(state, embed(|e| {
                e.title("Guild Joined!");
                e.description(guild.name.unwrap_or_default());
                e.color(Color::GOLD);
                e.timestamp_now();
            }))
            .await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    MyBot.run().await.unwrap();
}
