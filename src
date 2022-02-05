mod commands; 
mod handlers;
mod structures;

use std::{collections::HashSet, sync::Arc};

use serenity::{
    client::bridge::gateway::GatewayIntents, framework::standard::CommandResult, http::Http,
    Client
};

use structures::cmd_data::*;

use crate::handlers::{
    event_handler::{ SerenityHandler },
    framework::get_framework,
};


#[tokio::main]
async fn main() -> CommandResult {
    tracing_subscriber::fmt::init();

    let token = "СЮДА ТОКЕН ТВОЕГО БОТА";
    let prefix = "!"; // Можешь указать любой другой префикс, если хочешь

    let http = Http::new_with_token(token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Нет доступа к приложению, причина: {:?}", why),
    };

    let mut client = Client::builder(&token)
        .framework(get_framework(prefix, owners))
        .event_handler(SerenityHandler)
        .intents({
            GatewayIntents::GUILDS | GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::GUILD_MESSAGES
        })
        .await
        .expect("Ошибка при создании клиента :(");

    {
        let mut data = client.data.write().await;

        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<BotId>(bot_id);
    }

    if let Err(why) = client.start_autosharded().await {
        eprintln!("Ошибка при включении бота, причина: {:?}", why);
    }

    Ok(())
}
