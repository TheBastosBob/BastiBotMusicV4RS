use serenity::framework::standard::{macros::command, CommandResult};
use serenity::client::Context;

use serenity::{
    async_trait,
    framework::standard::{
        macros::{ group},
        Args,  CommandGroup, HelpOptions,
    },
    model::{gateway::Ready, id::ChannelId},
    model::prelude::Message,
    prelude::*,
};

use crate::utils::check_msg;


#[command]
#[only_in(guilds)]
pub async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg
        .guild(&ctx.cache)
        .unwrap();

    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        }
    };

    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.join(guild_id, connect_to).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        handler.stop();

        check_msg(msg.channel_id.say(&ctx.http, "Stopped").await);

    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to stop playback").await);
    }
    manager.remove(guild_id).await.expect("Leave failed");
    Ok(())
}