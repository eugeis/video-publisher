use reqwest::blocking::Client;
use serde_json::json;
use anyhow::Result;
use std::path::Path;

pub fn upload_to_telegram(bot_token: &str, chat_id: &str, file_path: &str, caption: &str) -> Result<()> {
    let client = Client::new();

    let form = reqwest::blocking::multipart::Form::new()
        .file("video", file_path)?;

    let url = format!("https://api.telegram.org/bot{}/sendVideo", bot_token);

    let res = client.post(&url)
        .multipart(form)
        .query(&[("chat_id", chat_id), ("caption", caption)])
        .send()?;

    println!("Telegram Upload Response: {:?}", res.text()?);
    Ok(())
}
