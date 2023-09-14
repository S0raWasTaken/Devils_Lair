use reqwest::Client;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    json::Value,
    model::prelude::Message,
    prelude::Context,
    utils::Color,
};
use std::env::var;

const COMMANDS: &str = "\
- **`status`**
- **`start`**
- **`stop`**
- **`restart`**
- **`kill`**";

#[command]
async fn status(ctx: &Context, msg: &Message) -> CommandResult {
    let mut handle = msg.reply_ping(&ctx.http, ":stopwatch:").await?;

    let api_url = var("API_URL")?;
    let api_key = var("API_KEY")?;
    let server_id = var("SERVER_ID")?;

    let request = Client::new()
        .get(format!("{api_url}/client/servers/{server_id}/resources"))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {api_key}"));

    let res = request.send().await?.json::<Value>().await?;

    let attributes = &res["attributes"];
    let resources = &attributes["resources"];

    let (days, hours, mins, _) = format_dhms(
        res["attributes"]["resources"]["uptime"]
            .as_u64()
            .ok_or("Tá faltando info de tempo na API, resolve aqui <@319637457907875841>")?
            / 1000,
    );

    let current_state = attributes["current_state"]
        .as_str()
        .ok_or("Tá faltando o estado do servidor <@319637457907875841>")?;
    let cpu_absolute = resources["cpu_absolute"]
        .as_f64()
        .ok_or("Não tem info da cpu <@319637457907875841>")?;
    let memory_bytes = resources["memory_bytes"]
        .as_f64()
        .ok_or("Sem info na ram <@319637457907875841>")?
        / 1_000_000_000.0;
    let disk_bytes = resources["disk_bytes"]
        .as_f64()
        .ok_or("Sem info no disco <@319637457907875841>")?
        / 1_000_000_000.0;

    let description = format!(
        r#"
        💻 State: {}
        💻 CPU: {cpu_absolute:.1}% / 300%
        💻 RAM: {memory_bytes:.1} GiB / 6 GiB
        💻 Disk: {disk_bytes:.1} GiB / ∞
        💻 Uptime: {days}d {hours}h {mins}m
        "#,
        current_state[0..1].to_uppercase() + &current_state[1..]
    );

    handle.edit(&ctx.http, |m| {
        m.content("").embed(|e| {
            e.title("Devil's Lair")
            .color(Color::FABLED_PINK)
            .description(description)
            .thumbnail("https://cdn.discordapp.com/attachments/725013161886875678/1151579930891669514/server-icon.png")
        })
    }).await?;

    Ok(())
}

#[command]
async fn start(ctx: &Context, msg: &Message) -> CommandResult {
    send_power_state("start").await?;

    msg.reply_ping(&ctx.http, "Start request sent").await?;
    Ok(())
}

#[command]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    send_power_state("stop").await?;

    msg.reply_ping(&ctx.http, "Stop request sent").await?;
    Ok(())
}

#[command]
async fn restart(ctx: &Context, msg: &Message) -> CommandResult {
    send_power_state("restart").await?;

    msg.reply_ping(&ctx.http, "Restart request sent").await?;
    Ok(())
}

#[command]
async fn kill(ctx: &Context, msg: &Message) -> CommandResult {
    send_power_state("kill").await?;

    msg.reply_ping(&ctx.http, "Kill request sent").await?;
    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("Comandos:")
                    .description(COMMANDS)
                    .color(Color::FABLED_PINK)
            })
            .reference_message(msg)
        })
        .await?;
    Ok(())
}

fn format_dhms(sec: u64) -> (u64, u8, u8, u8) {
    let (mins, sec) = (sec / 60, (sec % 60) as u8);
    let (hours, mins) = (mins / 60, (mins % 60) as u8);
    let (days, hours) = (hours / 24, (hours % 24) as u8);

    (days, hours, mins, sec)
}

async fn send_power_state(signal: &str) -> CommandResult {
    let api_url = var("API_URL")?;
    let api_key = var("API_KEY")?;
    let server_id = var("SERVER_ID")?;

    let request = Client::new()
        .post(format!("{api_url}/client/servers/{server_id}/power"))
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {api_key}"))
        .body(format!("{{ \"signal\": \"{signal}\" }}"));

    request.send().await?;

    Ok(())
}
