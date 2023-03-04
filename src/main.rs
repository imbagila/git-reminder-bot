use chrono::{Duration, Local};
use cron::Schedule;
use std::env;
use std::str::FromStr;
use std::thread::sleep;

const USER_AGENT_VAL: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/12.246";

fn main() {
    println!("git reminder bot is running - {}", Local::now().to_string());

    // setup env
    let gitlab_user_id = read_env("GITLAB_USER_ID");
    let gitlab_token = read_env("GITLAB_TOKEN");
    let telegram_bot_token = read_env("TELEGRAM_BOT_TOKEN");
    let telegram_chat_id = read_env("TELEGRAM_CHAT_ID");
    let message = read_env("MESSAGE");

    // setup cron: sec | min | hour | day of month | month | day of week | year
    let expression = "0 0 9,12,15,18,21 * * * *";
    let schedule = Schedule::from_str(expression).unwrap();

    // run cron loop
    loop {
        // setup delay
        let next = schedule.upcoming(Local).next().unwrap();
        let delay = next.signed_duration_since(Local::now()).to_std().unwrap();
        sleep(delay);

        // run jobs
        match get_contribution(&gitlab_user_id, &gitlab_token) {
            Ok(body) => {
                if body.contains("pushed") {
                    println!("contains");
                } else {
                    println!("not contains");
                    match send_tele(&telegram_bot_token, &telegram_chat_id, &message) {
                        Ok(_) => {
                            println!("success send tele");
                        }
                        Err(e) => eprintln!("failed to send tele: {}", e),
                    }
                }
            }
            Err(e) => eprintln!("error making request: {}", e),
        }
    }
}

// read required env, will exit if env is empty
fn read_env(key: &'static str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("couldn't read env {}: {}", key, e);
            std::process::exit(1);
        }
    }
}

// get contribution with gitlab api
fn get_contribution(gitlab_user_id: &String, gitlab_token: &String) -> Result<String, ureq::Error> {
    // setup date range
    let now = Local::now();
    let yesterday = (now - Duration::days(1)).format("%Y-%m-%d").to_string();
    let tomorrow = (now + Duration::days(1)).format("%Y-%m-%d").to_string();

    // gitlab api
    let gitlab_url = format!(
        "https://gitlab.com/api/v4/users/{}/events?after={}&before={}",
        gitlab_user_id, yesterday, tomorrow
    );

    // setup request
    let body: String = ureq::get(&gitlab_url)
        .set("User-Agent", USER_AGENT_VAL)
        .set("PRIVATE-TOKEN", gitlab_token)
        .call()?
        .into_string()?;

    Ok(body)
}

// send telegram notification
fn send_tele(
    telegram_bot_token: &String,
    telegram_chat_id: &String,
    message: &String,
) -> Result<(), ureq::Error> {
    let telegram_url = format!("https://api.telegram.org/bot{}/sendMessage?chat_id={}&parse_mode=html&disable_web_page_preview=1&text={}", telegram_bot_token, telegram_chat_id, message);

    ureq::get(&telegram_url)
        .set("User-Agent", USER_AGENT_VAL)
        .call()?
        .into_string()?;

    Ok(())
}
