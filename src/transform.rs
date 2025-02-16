use anyhow::{Result, Context};
use std::process::Command;

pub(crate) fn transform_video(file: &str) -> Result<String> {
    let output_file = format!("{}_reformatted.mp4", file);
    let status = Command::new("ffmpeg")
        .args(["-i", file, "-vf", "scale=480:-2", "-c:v", "libx264", "-preset", "fast", "-crf", "23", "-c:a", "copy", &output_file])
        .status()
        .context("Failed to run FFmpeg")?;

    if !status.success() {
        anyhow::bail!("FFmpeg processing failed");
    }
    Ok(output_file)
}
