use anyhow::{Context, Result};
use crate::rutube::upload_to_rutube;
use crate::telegram::upload_to_telegram;
use crate::vk::upload_to_vk;
use crate::Platform;

pub(crate) async fn upload(platform: Platform, file: &str, title: &str, rutube_api_key: Option<String>,
                           bot_api_url: &str, max_file_size: u64, bot_token: Option<String>,
                           chat_id: Option<i64>, vk_access_token: Option<String>,
                           message_before: &str, message_after: &str) -> Result<()> {
    match platform {
        Platform::Rutube => {
            let key = rutube_api_key.ok_or_else(|| anyhow::anyhow!("API key for Rutube is missing"))?;
            println!("Uploading '{}' to Rutube", file);
            upload_to_rutube(&key, file, title).await?;
        }
        Platform::Telegram => {
            let token = bot_token.ok_or_else(|| anyhow::anyhow!("Bot token for Telegram is missing"))?;
            let id = chat_id.ok_or_else(|| anyhow::anyhow!("Chat ID for Telegram is missing"))?;
            println!("Uploading '{}' to Telegram", file);
            upload_to_telegram(
                bot_api_url, max_file_size, &token, id, file, title, message_before, message_after)
                .await
                .context("Failed to upload video to Telegram")?;
        }
        Platform::Vk => {
            let token = vk_access_token.ok_or_else(|| anyhow::anyhow!("VK access token is missing"))?;
            println!("Uploading '{}' to VK", file);
            upload_to_vk(&token, title, file).await?;
        }
    }
    Ok(())
}
