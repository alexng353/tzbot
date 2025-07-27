use std::env;

use chrono::prelude::*;
use chrono_tz::Tz;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        match msg.content.as_str() {
            "!ping" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                    println!("Error sending message: {why:?}");
                }
            }
            "!tzs" => {
                // America/Vancouver, Singapore, Jakarta, Tokyo, Toronto.
                let tzs = vec![
                    "America/Vancouver",
                    "America/Toronto",
                    "Asia/Jakarta",
                    "Asia/Singapore",
                    "Asia/Tokyo",
                ];

                let mut out_string = String::from("```r\n");

                // Find the maximum length of time zone names for alignment
                let max_len = tzs.iter().map(|s| s.len()).max().unwrap_or(0);

                for tz_name in tzs {
                    let tz: Tz = tz_name.parse().expect("Invalid time zone");
                    let now = Utc::now().with_timezone(&tz);
                    let time_str = now.format("%m-%d %H:%M").to_string();
                    out_string.push_str(&format!(
                        "{:width$} - {}\n",
                        tz_name,
                        time_str,
                        width = max_len
                    ));
                }
                out_string.push_str("```");

                println!("{}", out_string);

                if let Err(why) = msg.channel_id.say(&ctx.http, out_string).await {
                    println!("Error sending message: {why:?}");
                }
            }
            _ => println!("Got a message from {}: {}", msg.author.name, msg.content),
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
