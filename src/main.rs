use chrono::{Datelike, Duration, Timelike};
use soup::{prelude::*, QueryBuilderExt, Soup};
mod creds;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().cookie_store(true).build()?;

    let res = client
        .get("https://kss.nesa-sg.ch/loginto.php?pageid=21312")
        .send()
        .await?;

    let body = res.text().await?;

    let soup = Soup::new(&body);

    let loginhash = soup
        .tag("input")
        .attr("name", "loginhash")
        .find()
        .expect("Couldn't get the loginhash")
        .get("value")
        .unwrap();

    // Get menu href
    let res = client
        .post("https://kss.nesa-sg.ch/index.php?pageid=21312")
        .body(format!(
            "login={}&passwort={}&loginhash={}",
            creds::creds::USERNAME,
            creds::creds::PASSWORD,
            loginhash
        ))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;

    let body = res.text().await?;

    let soup = Soup::new(&body);

    let href = soup
        .tag("a")
        .attr("id", "menu1")
        .find()
        .expect("Couldn't find the menu_href")
        .get("href")
        .unwrap();
    let href = href.split("=").collect::<Vec<&str>>();

    let id: &str = href[2].split("&").collect::<Vec<&str>>()[0];

    let trans_id: &str = href[3];

    static WEEKS: i64 = 2;

    let now = chrono::Utc::now();
    let start_date = now - Duration::days(now.weekday().num_days_from_monday() as i64);
    let end_date = start_date + Duration::days(7 * WEEKS - 1);

    let res = client
        .get("https://kss.nesa-sg.ch/scheduler_processor.php")
        .query(&[
            ("view", "week"),
            (
                "curr_date",
                &format!("{}-{}-{}", now.year(), now.month(), now.day()),
            ),
            (
                "min_date",
                &format!(
                    "{}-{}-{}",
                    start_date.year(),
                    start_date.month(),
                    start_date.day()
                ),
            ),
            (
                "max_date",
                &format!(
                    "{}-{}-{}",
                    end_date.year(),
                    end_date.month(),
                    end_date.day()
                ),
            ),
            ("ansicht", "klassenuebersicht"),
            ("id", id),
            ("transid", trans_id),
            ("pageid", "21312"),
            ("timeshift", "60"),
        ])
        .send()
        .await?;

    let body = res
        .text()
        .await?
        .replace("<data>", r#"<data xmlns="event">"#);

    let root: minidom::Element = body.parse().unwrap();

    let mut events: Vec<[String; 3]> = Vec::new();

    for event in root.children() {
        let start_date = event.get_child("start_date", "event").unwrap().text();
        let subject = event.get_child("text", "event").unwrap().text();
        let description = event.get_child("kommentar", "event").unwrap().text();

        events.push([
            start_date.split(" ").collect::<Vec<&str>>()[0].to_string(),
            subject.split("-").collect::<Vec<&str>>()[0].to_string(),
            description.clone(),
        ]);
    }

    events.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut body = String::new();

    for event in events {
        body.push_str(&format!("*{}* **{}** {} \\n", event[0], event[1], event[2]));
    }

    let _ = client
        .patch(format!(
            "https://discord.com/api/webhooks/{}/{}/messages/{}",
            creds::creds::WEBHOOK,
            creds::creds::TOKEN,
            creds::creds::MESSAGE_ID
        ))
        .header("Content-Type", "application/json")
        .body(format!(
            "{}{}{}{}{}",
            r#"{ "embeds": [{ "title": "Upcoming exams:", "color": "16711680","description": ""#,
            body,
            r#"","footer": {"text": "Last updated "#,
            format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second()),
            r#""}}]}"#
        ))
        .send()
        .await?;

    Ok(())
}
