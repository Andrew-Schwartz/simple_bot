use std::io::Write;
use std::sync::Arc;
use chrono::Local;

use discorsd::{Bot, BotExt, BotState};
use discorsd::commands::{SlashCommandRaw, UserCommand};
use discorsd::errors::BotError;
use discorsd::http::channel::{embed, MessageChannelExt};
use discorsd::model::guild::Guild;
use discorsd::model::message::Color;
use log::LevelFilter;
use config::*;

use crate::test_command::TestCommand;
use crate::test_user_command::TestUserCommand;

mod test_command;
mod echo_button;
mod menu_command;
mod config;
mod test_user_command;

pub struct MyBot;

#[discorsd::async_trait]
impl Bot for MyBot {
    fn token(&self) -> String {
        TOKEN.into()
    }

    fn global_commands() -> &'static [&'static dyn SlashCommandRaw<Bot=Self>] {
        &[&TestCommand]
    }

    fn global_user_commands() -> &'static [&'static dyn UserCommand<Bot=Self>] {
        &[&TestUserCommand]
    }

    async fn guild_create(&self, guild: Guild, state: Arc<BotState<Self>>) -> Result<(), BotError> {
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
    env_logger::builder()
        .format(|f, record|
            writeln!(f,
                     "{} [{}] {}",
                     Local::now().format("%e %T"),
                     record.level(),
                     record.args(),
            ))
        .filter(None, LevelFilter::Info)
        .init();

    MyBot.run().await.unwrap()
}
