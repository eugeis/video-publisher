use std::{process::Command, time::Duration};
use anyhow::{Result, anyhow};
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::Value;

pub fn download_video(url: &str, output: &str) -> Result<(String, String)> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner().template("{spinner} Downloading {msg}")?);
    pb.enable_steady_tick(Duration::from_millis(100));

    // Получаем метаданные с учетом кастомного формата имени файла
    let (filename, title) = get_video_metadata(url, output)?;

    //check if file already exists
    if std::path::Path::new(&filename).exists() {
        pb.finish_with_message("File already exists");
        return Ok((filename, title));
    }

    let status = Command::new("yt-dlp")
        .args([
            "-o",
            &format!("{}/%(title)s.%(ext)s", output),
            "-f", "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]",
            "--merge-output-format", "mp4",
            "--recode-video", "mp4",
            url,
        ])
        .status()?;

    pb.finish_with_message("Download complete");

    if !status.success() {
        return Err(anyhow!("Failed to download video"));
    }

    Ok((filename, title))
}

pub fn get_video_metadata(url: &str, output: &str) -> Result<(String, String)> {
    // Вызываем yt-dlp с нужным форматом имени файла
    let output_data = Command::new("yt-dlp")
        .arg("--dump-json")
        .arg("-o")
        .arg(format!("{}/%(title)s.%(ext)s", output))
        .arg(url)
        .arg("-f")
        .arg("bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]")
        .output()?;

    if !output_data.status.success() {
        return Err(anyhow!("Failed to fetch video metadata"));
    }

    // Парсим JSON
    let json_str = String::from_utf8_lossy(&output_data.stdout);
    let json: Value = serde_json::from_str(&json_str).map_err(|e| anyhow!("Failed to parse JSON: {}", e))?;

    // Получаем название и путь к файлу
    let mut title = json["title"].as_str().unwrap_or("Unknown Title").to_string();
    let filename = json["_filename"].as_str().unwrap_or("").to_string();

    if filename.is_empty() {
        return Err(anyhow!("Failed to determine filename"));
    }

    title = title.replace(" #shortvideo", "");

    Ok((filename, title))
}
