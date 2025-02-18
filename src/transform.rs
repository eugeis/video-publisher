use anyhow::{Result, Context};
use std::process::Command;

pub(crate) fn transform_video(file: &str) -> Result<String> {
    let output_file = format!("{}_reformatted.mp4", file);
    let status = Command::new("ffmpeg")
        .args([
            "-y",
            "-i", file,
            "-vf", "scale=-2:720",
            "-c:v", "libx264",
            "-preset", "slow",
            "-crf", "26",
            "-level", "4.1",
            "-pix_fmt", "yuv420p",
            "-c:a", "aac",
            "-b:a", "128k",
            "-movflags", "+faststart",
            &output_file
        ])
        .status()
        .context("Failed to run FFmpeg")?;

    if !status.success() {
        anyhow::bail!("FFmpeg processing failed");
    }
    Ok(output_file)
}
