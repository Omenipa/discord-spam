
This document explains how to build a Discord bot using Rust that can send images in response to a command. The bot is created using the `serenity` library and adheres to Discord’s API guidelines.

## Features
- Responds to a specific command (`!send_image`).
- Sends an image file along with a message.
- Supports message customization and loops for repeated actions.

---

## Prerequisites

1. **Rust Installed**: Install Rust from [rustup.rs](https://rustup.rs/).
2. **Discord Bot Token**: Create a bot on the [Discord Developer Portal](https://discord.com/developers/applications) and copy its token.
3. **Environment Variable Setup**: Store your bot token in an environment variable (`DISCORD_TOKEN`).

---


### 2. Bot Code

The following Rust code defines the bot. It uses `serenity` for Discord interactions and responds to commands with images.

#### **Code Explanation**
- **EventHandler**: Handles bot events like receiving messages.
- **!send_image Command**: Sends an image (`example_image.jpg`) and a message. Repeats this action a specified number of times.
- **Environment Variable**: The bot token is securely loaded from an environment variable.

```rust
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
            for i in 0..5 { // Sends 5 images
                if let Err(why) = msg
                    .channel_id
                    .send_message(&ctx.http, |m| {
                        m.content(format!("Image spam #{}", i + 1));
                        m.add_file("./images/example_image.jpg"); // Path to the image
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
```

---

### 3. Set Up Image Files

1. Create a directory named `images` in the project root.
2. Place an image file inside it (e.g., `example_image.jpg`).

---

### 4. Create a Discord Bot and Invite It

1. Go to the [Discord Developer Portal](https://discord.com/developers/applications).
2. Create a new application.
3. Navigate to the **Bot** tab and create a bot user.
4. Copy the bot token and set it as an environment variable:
   - On Linux/macOS:
     ```bash
     export DISCORD_TOKEN="your-bot-token"
     ```
   - On Windows (PowerShell):
     ```powershell
     $env:DISCORD_TOKEN="your-bot-token"
     ```
5. Generate an invite URL:
   - Navigate to **OAuth2 > URL Generator**.
   - Select the `bot` scope and grant `Send Messages` and `Attach Files` permissions.
   - Copy the generated URL and use it to invite the bot to your server.

---

### 5. Run the Bot

Start the bot by running the following command in the project directory:
```bash
cargo run
```

---

## Testing the Bot

1. In a Discord channel where the bot is present, type:
   ```
   !send_image
   ```
2. The bot will respond by sending the image (`example_image.jpg`) along with a message. It repeats the action 5 times in this example.

---

## Important Notes

### Rate Limits
Discord enforces API rate limits. Avoid spamming excessively as it violates Discord’s Terms of Service and can result in your bot being banned.

### Customization
- Modify the loop count (`for i in 0..5`) to adjust the number of repeated actions.
- Update the image file path (`./images/example_image.jpg`) to use a different image.



This bot provides a foundation for building more advanced features while staying compliant with Discord’s API guidelines.

## Made By jaaren


