use serenity::framework::standard::{macros::command, CommandResult};
use serenity::client::Context;

use serenity::{
    async_trait,
    client::bridge::gateway::ShardId,
    framework::StandardFramework,
    framework::standard::{
        macros::{group},
        Args, CommandGroup, HelpOptions,
    },
    model::{gateway::Ready, id::ChannelId},
    model::prelude::Message,
    prelude::*,
};

use songbird::SerenityInit;
use songbird::{
    input::{self
            , restartable::Restartable,
    },
    Event, Driver, EventContext, EventHandler as VoiceEventHandler, Songbird, TrackEvent, Call,
};


use url::Url;

use std::{sync::Arc, time::Duration,
          sync::{
              atomic::{AtomicUsize, Ordering},
          }, };
use std::alloc::handle_alloc_error;
use serenity::model::id::GuildId;
use songbird::driver::Bitrate;
use songbird::driver::opus::ffi::opus_get_version_string;
use crate::utils::check_msg;

#[command]
#[only_in(guilds)]
pub async fn pause(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg
        .guild(&ctx.cache)
        .unwrap();

    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();


    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();;
        let _ = queue.pause();

        check_msg(msg.channel_id.say(&ctx.http, format!("Song paused")).await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to pause in").await);
    }
    Ok(())
}
