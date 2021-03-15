use chrono::{Datelike, Duration, Timelike};

fn main() {
    static WEEKS: i64 = 2;

    let now = chrono::Utc::now();

    let start_date = now - Duration::days(now.weekday().num_days_from_monday() as i64);
    let end_date = start_date + Duration::days(7 * WEEKS - 1);

    println!(
        "Last updated: {:02}:{:02}:{:02}",
        now.hour(),
        now.minute(),
        now.second()
    );
    println!(
        "Start date: {}-{}-{}",
        start_date.year(),
        start_date.month(),
        start_date.day()
    );
    println!(
        "End date: {}-{}-{}",
        end_date.year(),
        end_date.month(),
        end_date.day()
    );
}
