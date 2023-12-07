// use std::sync::{Arc, Mutex};
// use serenity::model::prelude::{ChannelId, GuildId, Message};
// use songbird::{Call, Songbird};
//
// pub async fn check_msg(result: serenity::Result<Message>) {
//     if let Err(why) = result {
//         println!("Error sending message: {:?}", why);
//     }
// }
//
// pub struct Player {
//     guild_id: GuildId,
//     channel_id: ChannelId,
//     manager: Arc<Mutex<Songbird>>,
//     handler: Arc<Mutex<Call>>,
// }
//
