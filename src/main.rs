use chrono::prelude::*;
use chrono_tz::Tz;
use serenity::all::{ExecuteWebhook, Webhook};

fn build_tz_message() -> String {
    let tzs = vec![
        "America/Vancouver",
        "America/Toronto",
        "Asia/Jakarta",
        "Asia/Singapore",
        "Asia/Tokyo",
    ];

    let mut out_string = String::from("```r\n");
    let max_len = tzs.iter().map(|s| s.len()).max().unwrap_or(0) + 3;

    for tz_name in tzs {
        let tz: Tz = tz_name.parse().expect("Invalid time zone");
        let now = Utc::now().with_timezone(&tz);
        let time_str = now.format("%m-%d %H:%M").to_string();
        out_string.push_str(&format!("{tz_name:.<max_len$}{time_str}\n",));
    }

    out_string.push_str("```");
    out_string
}

async fn send_webhook(webhook_url: &str) {
    let http = serenity::http::Http::new("");
    let webhook = Webhook::from_url(&http, webhook_url)
        .await
        .expect("Err creating webhook");

    let builder = ExecuteWebhook::new()
        .content(build_tz_message())
        .username("World Clock");

    webhook
        .execute(&http, false, builder)
        .await
        .expect("Could not execute webhook.");
    println!("Sent webhook");
}

#[tokio::main]
async fn main() {
    let webhook_url = std::env::var("DISCORD_WEBHOOK").expect("No webhook url");
    send_webhook(&webhook_url).await;
}
