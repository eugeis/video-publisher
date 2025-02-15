use anyhow::{Result, Context};
use teloxide::prelude::*;
use teloxide::types::{ChatId, InputFile};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, BufReader};
use std::fs::metadata;

const CHUNK_SIZE: usize = 45 * 1024 * 1024; // 50MB per chunk (max for Telegram)

async fn send_video_chunk(bot: &Bot, chat_id: i64, video_path: &str, start: usize, end: usize) -> Result<()> {
    let file = File::open(video_path).await.context("Failed to open file")?;
    let mut reader = BufReader::new(file);

    let mut chunk = vec![0u8; end - start];
    reader.seek(std::io::SeekFrom::Start(start as u64)).await.context("Failed to seek to the start position")?;
    reader.read_exact(&mut chunk).await.context("Failed to read the chunk")?;

    let input_file = InputFile::memory(chunk);

    // Use .send() to actually send the video chunk and await the result
    bot.send_video(ChatId(chat_id), input_file)
        .await
        .context("Failed to upload chunk to Telegram")?;

    Ok(())
}

async fn upload_large_video(bot: &Bot, chat_id: i64, video_path: &str) -> Result<()> {
    let file_size = metadata(video_path)
        .map(|m| m.len())
        .context("Failed to get file metadata")?;

    let mut start = 0;
    let mut end = CHUNK_SIZE;

    // Loop through the file in chunks
    while start < file_size as usize {
        if end > file_size as usize {
            end = file_size as usize; // Don't go beyond the file size
        }

        println!("Uploading chunk: {} to {}", start, end);
        send_video_chunk(&bot, chat_id, video_path, start, end).await?;

        start = end;
        end += CHUNK_SIZE;
    }

    Ok(())
}

pub async fn upload_to_telegram(bot_token: &str, chat_id: i64, file_path: &str, caption: &str) -> Result<()> {
    let bot = Bot::new(bot_token);

    // Check the file size before deciding the upload method
    let file_size = metadata(file_path)
        .map(|m| m.len())
        .context("Failed to get file metadata")?;

    const MAX_FILE_SIZE: u64 = 50_000_000; // 50 MB limit for direct uploads

    if file_size > MAX_FILE_SIZE {
        // If the file is too large, use chunking
        upload_large_video(&bot, chat_id, file_path).await
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

    println!("Video uploaded successfully!");

    Ok(())
}
