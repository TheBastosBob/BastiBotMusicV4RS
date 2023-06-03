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
    , restartable::Restartable
    },
    Event,Driver, EventContext, EventHandler as VoiceEventHandler, Songbird, TrackEvent, Call,
};


use url::Url;

use std::{sync::Arc, time::Duration,
          sync::{
              atomic::{AtomicUsize, Ordering},
          },};
use std::alloc::handle_alloc_error;
use serenity::model::id::GuildId;
use songbird::driver::Bitrate;
use songbird::driver::opus::ffi::opus_get_version_string;
use crate::utils::check_msg;

pub struct EventConfig {
    pub ctx: Context,
    pub guild_id: GuildId,
    pub text_channel_id: ChannelId,
    pub voice_channel_id: ChannelId,
}

#[command]
#[only_in(guilds)]
pub async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild = msg
        .guild(&ctx.cache)
        .unwrap();

    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "Must provide a URL to a video or audio").await);

            return Ok(());
        }
    };

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



    if !url.starts_with("http") {
        check_msg(msg.channel_id.say(&ctx.http, "Must provide a valid URL").await);

        return Ok(());
    }

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler  = handler_lock.lock().await;
        let source = match Restartable::ytdl(url, true).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);

                return Ok(());
            }
        };


        handler.enqueue_source(source.into());

        let return_message;
        if handler.queue().len() > 1
        {
            return_message = format!("Added song : {} to queue at {}", handler.queue().current().unwrap().metadata().title.as_ref().unwrap(), handler.queue().len());
        } else {
            return_message = format!("Now playing: {}", handler.queue().current().unwrap().metadata().title.as_ref().unwrap());
        }

        check_msg(msg.channel_id.say(&ctx.http,  return_message).await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Not in a voice channel to play in").await);
    }



    Ok(())
}