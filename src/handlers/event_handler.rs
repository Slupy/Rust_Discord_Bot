use serenity::{
    async_trait,
    client::{ Context, EventHandler },
    model::prelude::Ready,
};

pub struct SerenityHandler;

#[async_trait]
impl EventHandler for SerenityHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} включён!", ready.user.name);
    }
}
