use serenity::{
    client::bridge::gateway::ShardManager,
    model::id::UserId,
    prelude::{ Mutex, TypeMapKey },
};

use std::sync::Arc;


pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct BotId;

impl TypeMapKey for BotId {
    type Value = UserId;
}
