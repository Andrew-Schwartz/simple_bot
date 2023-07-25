use std::io::Write;
use std::sync::Arc;
use chrono::Local;

use discorsd::{Bot, BotExt, BotState};
use discorsd::commands::SlashCommandRaw;
use discorsd::errors::BotError;
use discorsd::http::channel::{embed, MessageChannelExt};
use discorsd::model::guild::Guild;
use discorsd::model::ids::ChannelId;
use discorsd::model::message::Color;
use log::LevelFilter;

use crate::test_command::TestCommand;

mod test_command;
mod echo_button;
mod menu_command;
mod config;

const DEV_CHANNEL: ChannelId = ChannelId(780240796690808912);

pub struct MyBot;

#[discorsd::async_trait]
impl Bot for MyBot {
    fn token(&self) -> String {
        "NzgwMjM3MzE0NzM0Njg2MjA4.GTfO7_.pDC-G1q0wFjOj9T7PB12vOewuaUjF3I-kC3W8g".into()
    }

    fn global_commands() -> &'static [&'static dyn SlashCommandRaw<Bot=Self>] {
        &[&TestCommand]
    }

    async fn guild_create(&self, guild: Guild, state: Arc<BotState<Self>>) -> Result<(), BotError> {
        guild.channels.get(DEV_CHANNEL).unwrap()
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
