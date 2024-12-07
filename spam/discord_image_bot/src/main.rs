use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Command to trigger image sending
        if msg.content == "!send_image" {
            for i in 0..5 {
                if let Err(why) = msg
                    .channel_id
                    .send_message(&ctx.http, |m| {
                        m.content(format!("Image spam #{}", i + 1));
                        m.add_file("./images/example_image.jpg"); // Change to your image path
                        m
                    })
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot is connected as {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Load token from environment variable
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create bot client
    let mut client = Client::builder(&token, GatewayIntents::all())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Start the bot
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
