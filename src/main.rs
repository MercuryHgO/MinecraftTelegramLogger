use core::panic;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use std::time::{Duration, Instant};
use std::{thread, format};
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

    // Open the log file
    let file = File::open(log_path).expect("Could not open log file");
    let mut reader = BufReader::new(file);
    
    // Seek to the end of the file to start reading new entries
    let mut last_position = reader.seek(SeekFrom::End(0)).unwrap();

    loop {
        // Read the log file line by line
        let mut buffer = String::new();
        let bytes_read = reader.read_line(&mut buffer).unwrap();

        // If we reached the end of the file, wait and check again
        if bytes_read == 0 {
            thread::sleep(Duration::from_millis(100));
            // Seek to the last position to continue reading new entries
            reader.seek(SeekFrom::Start(last_position)).unwrap();
            continue;
        }

        // Update the last position
        last_position += bytes_read as u64;

        // Check for player join
        if let Some(captures) = join_regex.captures(&buffer) {
            if let Some(player_name) = captures.get(0) {
                send_telegram_message(
                    &bot_token, 
                    &chat_id, 
                    &format!(
                        "Player joined: {} \n", player_name.as_str())
                    )
                        .await;

                println!("Player joined: {}", player_name.as_str());
            }
        }

        // Check for player quit
        if let Some(captures) = quit_regex.captures(&buffer) {
            if let Some(player_name) = captures.get(0) {
                send_telegram_message(
                    &bot_token, 
                    &chat_id,
                    &format!(
                        "Player left: {}", player_name.as_str()
                    )
                ).await;
                println!("Player left: {}", player_name.as_str());
            }
        }

        // Sleep for a short duration to avoid busy waiting
        thread::sleep(Duration::from_millis(100));
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
