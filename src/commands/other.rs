use serenity::{
    client::Context,
    model::channel::Message,
    framework::standard::{ macros::command, CommandResult },
};

#[command]
#[aliases("пинг")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx, "Понг!")
        .await?;
    
    Ok(())
}
