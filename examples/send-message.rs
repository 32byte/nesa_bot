#[path = "../src/creds.rs"]
mod creds;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;

    let res = client
        .post(format!(
            "https://discord.com/api/webhooks/{}/{}",
            creds::creds::WEBHOOK,
            creds::creds::TOKEN
        ))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("content=Hello, I will be edited!")
        .send()
        .await?;

    assert!(res.status() == reqwest::StatusCode::NO_CONTENT);

    Ok(())
}
