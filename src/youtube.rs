use std::process::Command;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;
use anyhow::Result;

pub fn download_video(url: &str, output: &str) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner().template("{spinner} Downloading {msg}")?);
    pb.enable_steady_tick(Duration::from_millis(100));

    let status = Command::new("yt-dlp")
        .arg("-o")
        .arg(format!("{}/%(title)s.%(ext)s", output))
        .arg(url)
        .status()?;
    
    pb.finish_with_message("Download complete");
    
    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to download video"))
    }
}
