use chrono::{Datelike, Duration, Timelike};
use soup::{prelude::*, QueryBuilderExt, Soup};
mod creds;

/**
* How this works:
* This Webhook bot is split into 3 steps:
* 1. Login into Nesa
*   1.1 Getting the login hash required for the login
*   1.2 Login using the username, password and loginhash
* 2. Get the upcoming exams
*   2.1 Get the id and transid
*   2.2 Calculate the start- and end-date
*   2.3 Get the upcoming exams
* 3. Send the exams to the webhook
*   3.1 Parse the xml
*   3.2 Generate and send the Embed to the webhook
*/
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This is the http client, it is important that it stores cookies to be able to login
    let client = reqwest::Client::builder().cookie_store(true).build()?;

    // STEP 1.1: Get the loginhash
    // Firstly go to the login-page
    let res = client
        .get("https://kss.nesa-sg.ch/loginto.php?pageid=21312")
        .send()
        .await?;

    // Then get the html
    let body = res.text().await?;

    // Lastly parse it using the soup library
    let soup = Soup::new(&body);

    let loginhash = soup
        .tag("input")
        .attr("name", "loginhash")
        .find()
        .expect("Couldn't get the loginhash")
        .get("value")
        .unwrap();

    // STEP 1.2: login using username and password
    let res = client
        .post("https://kss.nesa-sg.ch/index.php?pageid=21312")
        .body(format!(
            // Note: The devs of the nesa-sg website are really inconsistent
            // Some parts are in english and some in german
            // I spent around 1h debugging why my requests wasn't working
            // and the reason was, I was using passworD instead of passworT
            "login={}&passwort={}&loginhash={}",
            creds::creds::USERNAME,
            creds::creds::PASSWORD,
            loginhash
        ))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;

    // STEP 2.1: Get the id and transid
    // We use the html from the last request to find the id and transid
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

    // STEP 2.2: Calculate the start and end date
    static WEEKS: i64 = 2;

    let now = chrono::Utc::now();
    let start_date = now - Duration::days(now.weekday().num_days_from_monday() as i64);
    let end_date = start_date + Duration::days(7 * WEEKS - 1);

    // STEP 2.3: Make a request to get the upcoming exams
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

    // Step 3.1: parse the received xml
    let body = res
        .text()
        .await?
        // Note: minidom needs a xmlns (xml namespace) to work
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

    // Sort the events by date because somehow the server doesn't send them sorted.. -.-
    events.sort_by(|a, b| a[0].cmp(&b[0]));

    // STEP 3.2: Generate and send the embed
    let mut body = String::new();

    // This generates the embed description
    for event in events {
        body.push_str(&format!("*{}* **{}** {} \\n", event[0], event[1], event[2]));
    }

    // Send the embed
    let _ = client
        .patch(format!(
            "https://discord.com/api/webhooks/{}/{}/messages/{}",
            creds::creds::WEBHOOK,
            creds::creds::TOKEN,
            creds::creds::MESSAGE_ID
        ))
        .header("Content-Type", "application/json")
        // Note: this body part is really ugly to read, so if anyone has improvements then just make a pr
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
