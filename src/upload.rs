use anyhow::Context;
use crate::rutube::upload_to_rutube;
use crate::telegram::upload_to_telegram;
use crate::vk::upload_to_vk;
use crate::insta::upload_to_instagram;
use anyhow::Result;

pub(crate) async fn upload(platform: &str, file: &str, title: &str, rutube_api_key: Option<String>,
                           bot_api_url: &str, max_file_size: u64, bot_token: Option<String>,
                           chat_id: Option<i64>, vk_access_token: Option<String>,
                           instagram_username: Option<String>,
                           instagram_password: Option<String>,
                           message_before: &str, message_after: &str) -> Result<()> {
    match platform {
        "rutube" => {
            if let Some(key) = rutube_api_key {
                println!("Uploading '{}' to Rutube", file);
                upload_to_rutube(&key, &file, &title)?;
            } else {
                println!("API key for Rutube is missing.");
            }
        }
        "telegram" => {
            if let (Some(token), Some(id)) = (bot_token, chat_id) {
                println!("Uploading '{}' to Telegram", file);
                // Await the asynchronous upload function
                upload_to_telegram(
                    &bot_api_url, max_file_size, &token, id, file, title, message_before, message_after)
                    .await.context("Failed to upload video to Telegram")?;
            } else {
                println!("Bot token or chat ID for Telegram is missing.");
            }
        }
        "vk" => {
            if let Some(token) = vk_access_token {
                println!("Uploading '{}' to VK", file);
                upload_to_vk(&token, &title, &file)?;
            } else {
                println!("VK access token is missing.");
            }
        }
        "instagram" => {
            if let (Some(username), Some(password)) = (instagram_username, instagram_password) {
                println!("Uploading '{}' to Instagram", file);
                upload_to_instagram(&file, &username, &password).await
                    .context("Failed to upload video to Instagram")?;
            } else {
                println!("Instagram username or password is missing.");
            }
        }
        _ => println!("Unsupported platform: {}", platform),
    }
    Ok(())
}