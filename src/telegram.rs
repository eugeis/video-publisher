use anyhow::{Result, Context};
use teloxide::prelude::*;
use teloxide::types::{ChatId, InputFile};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, BufReader};
use std::fs::metadata;
use teloxide::net;

async fn upload_large_video(
    max_file_size: u64, bot: &Bot, chat_id: i64, video_path: &str) -> Result<()> {

    let file_size = metadata(video_path)
        .map(|m| m.len())
        .context("Failed to get file metadata")?;

    let mut start = 0;
    let mut end = max_file_size;

    let file = File::open(video_path).await.context("Failed to open file")?;
    let mut reader = BufReader::new(file);

    let mut chunks: Vec<Vec<u8>> = Vec::new();

    // Loop through the file in chunks
    while start < file_size {
        if end > file_size {
            end = file_size;
        }

        let mut chunk = vec![0u8; (end - start) as usize];
        reader.seek(std::io::SeekFrom::Start(start as u64)).await.context("Failed to seek to the start position")?;
        reader.read_exact(&mut chunk).await.context("Failed to read the chunk")?;

        chunks.push(chunk);

        start = end;
        end += max_file_size;
    }

    // Send the file in chunks
    for chunk in chunks {
        let input_file = InputFile::memory(chunk);
        bot.send_video(ChatId(chat_id), input_file)
            .await
            .context("Failed to upload chunk to Telegram")?;
    }
    Ok(())
}

pub async fn upload_to_telegram(
    bot_url: &str, max_file_size: u64, bot_token: &str, chat_id: i64, file_path: &str,
    caption: &str, message_before: &str, message_after: &str) -> Result<()> {

    let client = net::default_reqwest_settings()
        .timeout(std::time::Duration::from_secs(240)).build().expect("Client creation failed");

    let bot = Bot::with_client(bot_token, client).set_api_url(bot_url.parse()?);


    // Check the file size before deciding the upload method
    let file_size = metadata(file_path)
        .map(|m| m.len())
        .context("Failed to get file metadata")?;

    if message_before != "" {
        bot.send_message(ChatId(chat_id), message_before).send().await?;
    }

    if file_size > max_file_size {
        // If the file is too large, use chunking
        upload_large_video(max_file_size, &bot, chat_id, file_path).await
            .context("Failed to upload video in chunks")?;
    } else {
        // Directly send the video if the file size is within the limit
        let input_file = InputFile::file(file_path);
        // Make sure to call .send() and then await the result
        bot.send_video(ChatId(chat_id), input_file)
            .caption(caption)
            .send()  // Send the request asynchronously
            .await
            .context("Failed to upload video to Telegram")?;
    }

    if message_after != "" {
        bot.send_message(ChatId(chat_id), message_after).send().await?;
    }

    println!("Video uploaded successfully!");

    Ok(())
}