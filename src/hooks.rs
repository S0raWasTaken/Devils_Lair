use std::env::var;

use serenity::{
    framework::standard::{macros::hook, CommandError},
    model::prelude::Message,
    prelude::Context,
};

#[hook]
pub async fn pre_command_checks(_ctx: &Context, message: &Message, _: &str) -> bool {
    Some(message.channel_id.as_u64().to_string()) == var("TARGET_CHANNEL").ok()
}

#[hook]
pub async fn post_command(
    ctx: &Context,
    message: &Message,
    _: &str,
    result: Result<(), CommandError>,
) {
    println!(
        "Successful command from {}: {}",
        message.author.tag(),
        message.content
    );
    if let Err(why) = result {
        message
            .reply_ping(&ctx.http, why.to_string().to_code_block("yml"))
            .await
            .ok();
    }
}

pub trait ToCodeBlock {
    fn to_code_block(&self, t: &str) -> String;
}

impl ToCodeBlock for String {
    fn to_code_block(&self, t: &str) -> String {
        format!("```{t}\n{self}```")
    }
}
