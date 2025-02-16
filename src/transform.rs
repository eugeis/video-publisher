use anyhow::{Result, Context};
use std::process::Command;

pub(crate) fn transform_video(file: &str) -> Result<String> {
    let output_file = format!("{}_reformatted.mp4", file);
    let status = Command::new("ffmpeg")
        .args([
            "-i", file,
            "-vf", "scale=1280:-2",  // 720p (или 480:-2 для более легких видео)
            "-c:v", "libx264",
            "-preset", "slow",        // Лучше качество при небольшом размере
            "-crf", "23",             // Баланс между качеством и размером
            "-profile:v", "high",
            "-level", "4.2",          // Максимальная совместимость с Telegram
            "-pix_fmt", "yuv420p",
            "-c:a", "aac",            // Кодек AAC (более совместимый)
            "-b:a", "128k",           // Оптимальный битрейт для аудио
            "-movflags", "+faststart", // Ускоряет начало воспроизведения
            &output_file
        ])
        .status()
        .context("Failed to run FFmpeg")?;

    if !status.success() {
        anyhow::bail!("FFmpeg processing failed");
    }
    Ok(output_file)
}
