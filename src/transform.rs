use anyhow::{Result, Context};
use std::process::Command;

pub(crate) struct Metadata {
    pub width: i32,
    pub height: i32,
    pub duration: f64,
}

pub(crate) fn transform_video(file: &str) -> Result<String> {
    let output = Command::new("ffmpeg")
        .arg("-hide_banner")
        .arg("-i")
        .arg(file)
        .output()
        .context("Failed to run FFmpeg for metadata")?;

    let metadata = parse_metadata(&output.stdout)?;
    let is_portrait = metadata.width < metadata.height;
    let duration_seconds = metadata.duration;

    let output_file = format!("{}_compressed.mp4", file);

    let target_size_mb: f64 = 300.0; // Maximale Zielgröße
    let avg_bitrate = ((target_size_mb * 8_000.0) / duration_seconds).min(2500.0).max(500.0) as i32;
    let max_bitrate = (avg_bitrate as f64 * 1.5) as i32;

    let (preset, crf, scale) = if duration_seconds < 300.0 {
        ("veryslow", "21", if is_portrait { "scale=1280:-2" } else { "scale=-2:1080" })
    } else {
        ("veryslow", "23", if is_portrait { "scale=1280:-2" } else { "scale=-2:1080" })
    };

    let pass_log = format!("{}_log", file);

    // **Erster Durchgang: Analyse**
    Command::new("ffmpeg")
        .args([
            "-y", "-i", file,
            "-vf", scale,
            "-c:v", "libx264",
            "-preset", preset,
            "-b:v", &format!("{}k", avg_bitrate),
            "-maxrate", &format!("{}k", max_bitrate),
            "-bufsize", &format!("{}k", max_bitrate * 2),
            "-pass", "1",
            "-an",
            "-f", "mp4",
            "/dev/null",
        ])
        .status()
        .context("Failed in first pass")?;

    // **Zweiter Durchgang: Encoding**
    Command::new("ffmpeg")
        .args([
            "-y", "-i", file,
            "-vf", scale,
            "-c:v", "libx264",
            "-preset", preset,
            "-b:v", &format!("{}k", avg_bitrate),
            "-maxrate", &format!("{}k", max_bitrate),
            "-bufsize", &format!("{}k", max_bitrate * 2),
            "-pass", "2",
            "-c:a", "aac",
            "-b:a", "128k",
            "-movflags", "+faststart",
            &output_file,
        ])
        .status()
        .context("Failed in second pass")?;

    Ok(output_file)
}

pub(crate) fn parse_metadata(output: &[u8]) -> Result<Metadata> {
    let stdout = String::from_utf8_lossy(output);

    let mut width = 0;
    let mut height = 0;
    let mut duration = 0.0;

    for line in stdout.lines() {
        if line.contains("width=") && line.contains("height=") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            for part in parts {
                if part.starts_with("width=") {
                    width = part.split("=").nth(1).unwrap_or("0").parse::<i32>().unwrap_or(0);
                }
                if part.starts_with("height=") {
                    height = part.split("=").nth(1).unwrap_or("0").parse::<i32>().unwrap_or(0);
                }
            }
        }
        if line.contains("Duration:") {
            let time_str = line.split(": ").nth(1).unwrap_or("");
            let mut dur_seconds = 0.0;
            if let Some((h, m_s)) = time_str.split_once(':') {
                let (m, s) = m_s.split_once(':').unwrap();
                dur_seconds = h.parse::<f64>().unwrap_or(0.0) * 3600.0 +
                    m.parse::<f64>().unwrap_or(0.0) * 60.0 +
                    s.parse::<f64>().unwrap_or(0.0);
            }
            duration = dur_seconds;
        }
    }

    Ok(Metadata {
        width,
        height,
        duration,
    })
}