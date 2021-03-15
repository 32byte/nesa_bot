#[path = "../src/creds.rs"]
mod creds;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;

    let events = vec![("12.12.", "M", "Exam1"), ("12.12.", "L", "Exam 7")];

    let mut body = String::new();

    for event in events {
        body.push_str(&format!("*{}* **{}** {} \\n", event.0, event.1, event.2));
    }

    let res = client
        .patch(format!(
            "https://discord.com/api/webhooks/{}/{}/messages/{}",
            creds::creds::WEBHOOK,
            creds::creds::TOKEN,
            creds::creds::MESSAGE_ID
        ))
        .header("Content-Type", "application/json")
        .body(format!(
            "{}{}{}",
            r#"{ "embeds": [{ "title": "Upcomming exams:", "color": "16711680","description": ""#,
            body,
            r#"","footer": {"text": "Last updated @11:00"}}]}"#
        ))
        .send()
        .await?;

    assert!(res.status() == reqwest::StatusCode::OK);

    Ok(())
}
