use anyhow::{Result, Context};
use std::process::Command;

pub(crate) fn transform_video(file: &str) -> Result<String> {
    let output_file = format!("{}_reformatted.mp4", file);
    let status = Command::new("ffmpeg")
        .args([
            "-i", file,
            // Apply rotation and scale the video
            "-vf", "transpose=1,scale=1280:-2",
            "-c:v", "libx264",
            "-preset", "slow",        // Better quality at a smaller size
            "-crf", "23",             // Balance between quality and size
            "-profile:v", "high",
            "-level", "4.1",          // Maximum compatibility with Telegram
            "-pix_fmt", "yuv420p",
            "-c:a", "aac",            // AAC codec (more compatible)
            "-b:a", "128k",           // Optimal bitrate for audio
            "-movflags", "+faststart", // Speeds up playback start
            // Remove rotation metadata
            "-metadata:s:v:0", "rotate=0",
            &output_file
        ])
        .status()
        .context("Failed to run FFmpeg")?;

    if !status.success() {
        anyhow::bail!("FFmpeg processing failed");
    }
    Ok(output_file)
}