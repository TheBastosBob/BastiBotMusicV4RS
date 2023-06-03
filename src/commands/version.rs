use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::Context;


#[command]
pub async fn version(ctx: &Context, msg: &Message) -> CommandResult {

    let version = env!("CARGO_PKG_VERSION");
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
          m.embed(|e| {
              e.title("BastiBotMusicV4RS");
              e.description("Version");
                e.field("", version, true)
          });
            m
    }).await;

    Ok(())
}
