use chrono::prelude::*;
use chrono_tz::Tz;
use serenity::all::{ExecuteWebhook, Webhook};
use tokio_cron_scheduler::{Job, JobScheduler};

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
        out_string.push_str(&format!(
            "{:.<width$}{}\n",
            tz_name,
            time_str,
            width = max_len
        ));
    }

    out_string.push_str("```");
    out_string
}

async fn send_webhook() {
    let http = serenity::http::Http::new("");
    let webhook = Webhook::from_url(
                &http,
                "https://discord.com/api/webhooks/1401706619473100924/tmqza36wAWHNawp1OYUTaktQ8fjA4tpExbWsMcp4obn7zLkVCDFBk6qZUNazpfuKmFPD",
            ).await.expect("Err creating webhook");

    let builder = ExecuteWebhook::new()
        .content(build_tz_message())
        .username("Webhook test");

    webhook
        .execute(&http, false, builder)
        .await
        .expect("Could not execute webhook.");
    println!("Sent webhook");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    send_webhook().await;

    let sched = JobScheduler::new().await?;
    let job = Job::new_async("0 0,30 * * * *", |_uuid, _l| {
        Box::pin(async move {
            send_webhook().await;
        })
    })
    .expect("Failed to create cron job");
    sched.add(job).await?;

    sched.start().await?;
    tokio::time::sleep(std::time::Duration::from_secs(100)).await;
    Ok(())
}
