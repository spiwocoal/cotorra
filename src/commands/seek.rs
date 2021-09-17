use std::time::Duration;

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult, Delimiter},
    model::channel::Message,
};

#[command]
async fn seek(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild(&ctx.cache).await.unwrap().id;
    let manager = songbird::get(ctx).await.expect("Could not retrieve Songbird voice client");

    if let Some(call) = manager.get(guild_id) {
        let handler = call.lock().await;

        let seek_time = match args.single::<String>() {
            Ok(t) => t,
            Err(_) => {
                msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| e.description("Include a timestamp!"))
                }).await?;
                return Ok(());
            }
        };

        let mut timestamp = Args::new(&seek_time, &[Delimiter::Single(':')]);
        let (minutes, seconds) = (timestamp.single::<u64>(), timestamp.single::<u64>());

        if minutes.as_ref().and(seconds.as_ref()).is_err() {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| e.description("Could not parse timestamp!"))
            }).await?;
            return Ok(());
        }

        let timestamp = minutes.unwrap() * 60 + seconds.unwrap();

        let track = handler.queue().current().expect("Failed to fetch handle for current track");
        track.seek_time(Duration::from_secs(timestamp)).expect("Failed to seek on track");

        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| e.description(format!("Seeked current track to **{}**!", seek_time)))
        }).await?;
    }
    else {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| e.description("I'm not connected to any voice channel!"))
        }).await?;
    }

    Ok(())
}
