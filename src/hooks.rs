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
        let why = why.to_string();
        let error_context = why.split(" ><>< ").collect::<Vec<_>>();
        let (generic, context) = (error_context[0], error_context[1]);

        let content = format!(
            "{}\nContexto: ```json\n{context}```",
            generic.to_string().to_code_block("yml")
        );

        message.reply_ping(&ctx.http, content).await.ok();
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
