use std::{env::var, error::Error};

use commands::*;
use dotenv::dotenv;
use hooks::{post_command, pre_command_checks};
use serenity::{
    async_trait,
    framework::{standard::macros::group, StandardFramework},
    model::prelude::{Ready, UserId},
    prelude::{Context, EventHandler, GatewayIntents},
    Client,
};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is in!", ready.user.name);
    }
}

#[group]
#[commands(status, start, stop, restart, kill, help)]
struct Console;

mod commands;
mod hooks;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let token = var("DISCORD_TOKEN")?;
    let app_id = UserId::from(var("APPLICATION_ID")?.parse::<u64>()?);

    let framework = StandardFramework::new()
        .configure(|c| {
            c.on_mention(Some(app_id))
                .delimiters([", ", ",", " "])
                .allow_dm(false)
                .with_whitespace(true)
                .prefix(">>")
        })
        .before(pre_command_checks)
        .after(post_command)
        .group(&CONSOLE_GROUP);

    let mut client = Client::builder(token, GatewayIntents::all())
        .event_handler(Handler)
        .framework(framework)
        .application_id(*app_id.as_u64())
        .await?;

    if let Err(why) = client.start().await {
        eprintln!("{why}");
    }

    Ok(())
}
