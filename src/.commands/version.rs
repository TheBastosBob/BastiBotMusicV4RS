use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::Context;


#[command]
pub async fn version(ctx: &Context, msg: &Message) -> CommandResult {


    Ok(())
}
