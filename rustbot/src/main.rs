use serenity::async_trait;
use serenity::client::{Client, EventHandler};
use serenity::framework::StandardFramework;
use serenity::prelude::*;
use serenity::model::prelude::Ready;
use serenity::model::id::ChannelId;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("Bot is ready");

        let channel_id: u64 = std::env::var("DISCORD_CHANNEL_ID")
            .expect("Expected a channel ID in the environment")
            .parse()
            .expect("Channel ID should be a valid u64");

        let channel = ChannelId(channel_id);
        let _ = channel.say(&ctx.http, "Hello, team!").await;
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"));  // Setting bot's prefix to "~"

    let mut client = Client::builder(token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
