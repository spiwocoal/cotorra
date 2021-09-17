use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
#[aliases("s")]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild(&ctx.cache).await.unwrap().id;
    let manager = songbird::get(ctx).await.expect("Could not retrieve Songbird voice client");

    if let Some(call) = manager.get(guild_id) {
        let handler = call.lock().await;

        if handler.queue().is_empty() {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| e.description("The queue is already empty!"))
            }).await?;
        }
        else if handler.queue().skip().is_ok() {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| e.description("Skipped!"))
            }).await?;
        }
    } else {
        msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| e.description("I'm not connected to any voice channel!"))
        }).await?;
    }

    Ok(())
}
