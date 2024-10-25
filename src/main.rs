use core::panic;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use std::time::{Duration, Instant};
use std::{thread, format};
use notify::{Watcher, Config};
use regex::Regex;
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() {
    let log_path: &str = &std::env::var("LOG_PATH")
        .expect("LOG_PATH");
    let bot_token = &std::env::var("BOT_TOKEN")
        .expect("BOT_TOKEN");
    let chat_id = &std::env::var("CHAT_ID")
        .expect("CHAT_ID");

    // Regular expressions to match join and quit events
    let join_regex = Regex::new(r"\[Server thread\/INFO\]: (.*) joined the game").unwrap();
    let quit_regex = Regex::new(r"\[Server thread\/INFO\]: (.*) left the game").unwrap();

    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = notify::recommended_watcher(tx).unwrap();

    watcher.watch(Path::new(log_path), notify::RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => {
                if let Some(path) = event.unwrap().paths.get(0) {
                    if path.to_string_lossy() == log_path {
                        // Read the new lines from the log file
                        process_log_file(&log_path, &join_regex, &quit_regex, &bot_token, &chat_id).await;
                    }
                }
            }
            Err(e) => eprintln!("Watch error: {:?}", e),
        }
    }
}

async fn process_log_file(log_path: &str, join_regex: &Regex, quit_regex: &Regex, bot_token: &str, chat_id: &str) {
    let file = File::open(log_path).expect("Could not open log file");
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        match line {
            Ok(buffer) => {
                // Check for player join
                if let Some(captures) = join_regex.captures(&buffer) {
                    if let Some(player_name) = captures.get(1) {
                        send_telegram_message(
                            bot_token,
                            chat_id,
                            &format!("Player joined: {}", player_name.as_str())
                        ).await;
                        println!("Player joined: {}", player_name.as_str());
                    }
                }

                // Check for player quit
                if let Some(captures) = quit_regex.captures(&buffer) {
                    if let Some(player_name) = captures.get(1) {
                        send_telegram_message(
                            bot_token,
                            chat_id,
                            &format!("Player left: {}", player_name.as_str())
                        ).await;
                        println!("Player left: {}", player_name.as_str());
                    }
                }
            }
            Err(e) => eprintln!("Error reading line: {:?}", e),
        }
    }
}

async fn send_telegram_message(
    bot_token: &str,
    chat_id: &str,
    message: &str
) {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Error building client");

    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let params = json!({
        "chat_id": chat_id,
        "text": message
    });

    let response = client.post(&url)
        .json(&params)
        .send()
        .await
        .expect("Failed to send Telegram message");

    if !response.status().is_success() {
        eprintln!("Failed to send Telegram message: {:?}", response.status());
    }   
}
