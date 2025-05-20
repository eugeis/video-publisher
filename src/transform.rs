use anyhow::{Result, Context};
use std::process::Command;
use std::ffi::OsStr; // Needed for Command::args

pub(crate) struct Metadata {
    pub width: i32,
    pub height: i32,
    pub duration: f64,
}

// Optional configuration to choose between two-pass or one-pass encoding
pub(crate) enum EncodingPasses {
    TwoPass,
    OnePassCrf,
}

pub(crate) fn transform_video(file: &str, encoding_passes: EncodingPasses) -> Result<String> {
    // Use ffprobe for metadata extraction. It's more robust and cleaner than parsing ffmpeg's output.
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("error") // Suppress verbose output, only show errors
        .arg("-select_streams")
        .arg("v:0") // Select only the first video stream
        .arg("-show_entries")
        .arg("stream=width,height:format=duration") // Show width, height from stream, and duration from format
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1") // Output in a clean, key-value format (values only, no keys)
        .arg(file)
        .output()
        .context("Failed to run FFprobe for metadata. Is ffprobe installed and in PATH?")?;


    let metadata = parse_ffprobe_output(&output.stdout)?; // Adjusted parsing function
    let is_portrait = metadata.width < metadata.height;
    let duration_seconds = metadata.duration;

    let output_file = format!("{}_compressed.mp4", file);

    // Retain your existing logic for bitrate and scaling based on video duration
    let target_size_mb: f64 = 300.0; // Maximum target size in MB for bitrate calculation
    let avg_bitrate = ((target_size_mb * 8_000.0) / duration_seconds).min(2500.0).max(500.0) as i32;
    let max_bitrate = (avg_bitrate as f64 * 1.5) as i32;

    // Determine preset, CRF value, and scaling based on duration
    let (preset, crf_value, scale) = if duration_seconds < 300.0 {
        ("veryslow", "21", if is_portrait { "scale=1280:-2" } else { "scale=-2:1080" })
    } else {
        ("veryslow", "23", if is_portrait { "scale=1280:-2" } else { "scale=-2:1080" })
    };

    // Common video encoding arguments for iOS compatibility
    // Store these as Strings because some will be formatted later
    let common_video_args: Vec<String> = vec![
        "-c:v".into(), "libx264".into(),
        "-profile:v".into(), "high".into(), // Crucial for broad iOS compatibility
        "-level:v".into(), "4.2".into(),    // Crucial for broad iOS compatibility
        "-pix_fmt".into(), "yuv420p".into(), // Crucial for broad iOS compatibility (8-bit, 4:2:0 subsampling)
        "-preset".into(), preset.into(),
        "-vf".into(), scale.into(), // Video filter for scaling
    ];

    // Common audio encoding arguments
    let common_audio_args: Vec<String> = vec![
        "-c:a".into(), "aac".into(), // AAC is the standard audio codec for MP4 and widely supported
        "-b:a".into(), "128k".into(), // Audio bitrate
    ];

    // Common output arguments
    let common_output_args: Vec<String> = vec![
        "-movflags".into(), "+faststart".into(), // Places metadata at the beginning for faster playback and preview generation
        "-y".into(), // Overwrite output file without asking
        output_file.clone().into(), // Clone output_file String
    ];

    match encoding_passes {
        EncodingPasses::TwoPass => {
            // **First Pass: Analysis (with iOS compatibility parameters)**
            let mut pass1_args: Vec<String> = vec![
                "-i".into(), file.into(), // Input file
            ];
            pass1_args.extend_from_slice(&common_video_args);
            // Add bitrate and pass-specific arguments as owned Strings
            pass1_args.push("-b:v".into());
            pass1_args.push(format!("{}k", avg_bitrate)); // Now owns the String
            pass1_args.push("-maxrate".into());
            pass1_args.push(format!("{}k", max_bitrate)); // Now owns the String
            pass1_args.push("-bufsize".into());
            pass1_args.push(format!("{}k", max_bitrate * 2)); // Now owns the String
            pass1_args.extend_from_slice(&[
                "-pass".into(), "1".into(), // Indicate first pass
                "-an".into(), // No audio in the first pass
                "-f".into(), "mp4".into(), // Output format (even if to null device)
                "/dev/null".into(), // Output to null device (discard video output)
            ]);

            // Command::args takes an iterator of AsRef<OsStr>, which &String implements
            Command::new("ffmpeg")
                .args(pass1_args.iter().map(|s| s.as_ref() as &OsStr))
                .status()
                .context("Failed in first pass of FFmpeg encoding")?;

            // **Second Pass: Encoding (with iOS compatibility parameters)**
            let mut pass2_args: Vec<String> = vec![
                "-i".into(), file.into(), // Input file
            ];
            pass2_args.extend_from_slice(&common_video_args);
            // Add bitrate and pass-specific arguments as owned Strings
            pass2_args.push("-b:v".into());
            pass2_args.push(format!("{}k", avg_bitrate)); // Now owns the String
            pass2_args.push("-maxrate".into());
            pass2_args.push(format!("{}k", max_bitrate)); // Now owns the String
            pass2_args.push("-bufsize".into());
            pass2_args.push(format!("{}k", max_bitrate * 2)); // Now owns the String
            pass2_args.extend_from_slice(&[
                "-pass".into(), "2".into(), // Indicate second pass
            ]);
            pass2_args.extend_from_slice(&common_audio_args);
            pass2_args.extend_from_slice(&common_output_args);

            // Command::args takes an iterator of AsRef<OsStr), which &String implements
            Command::new("ffmpeg")
                .args(pass2_args.iter().map(|s| s.as_ref() as &OsStr))
                .status()
                .context("Failed in second pass of FFmpeg encoding")?;
        },
        EncodingPasses::OnePassCrf => {
            // **Single Pass with CRF (simpler, but less precise control over file size)**
            let mut onepass_args: Vec<String> = vec![
                "-i".into(), file.into(), // Input file
            ];
            onepass_args.extend_from_slice(&common_video_args);
            // Add CRF argument as owned String
            onepass_args.push("-crf".into());
            onepass_args.push(crf_value.into()); // Now owns the String
            onepass_args.extend_from_slice(&common_audio_args);
            onepass_args.extend_from_slice(&common_output_args);

            // Command::args takes an iterator of AsRef<OsStr), which &String implements
            Command::new("ffmpeg")
                .args(onepass_args.iter().map(|s| s.as_ref() as &OsStr))
                .status()
                .context("Failed in one-pass FFmpeg encoding with CRF")?;
        }
    }

    Ok(output_file)
}

// Adjusted parsing function for ffprobe output
pub(crate) fn parse_ffprobe_output(output: &[u8]) -> Result<Metadata> {
    let stdout = String::from_utf8_lossy(output);
    let mut lines = stdout.lines();

    let width_str = lines.next().context("Missing width in ffprobe output")?;
    let height_str = lines.next().context("Missing height in ffprobe output")?;
    let duration_str = lines.next().context("Missing duration in ffprobe output")?;

    let width = width_str.parse::<i32>().context(format!("Failed to parse width: {}", width_str))?;
    let height = height_str.parse::<i32>().context(format!("Failed to parse height: {}", height_str))?;
    let duration = duration_str.parse::<f64>().context(format!("Failed to parse duration: {}", duration_str))?;

    // Basic validation to ensure metadata was parsed correctly
    if width == 0 || height == 0 || duration == 0.0 {
        return Err(anyhow::anyhow!("Parsed width, height, or duration is zero. Width: {}, Height: {}, Duration: {}. Full output: {}", width, height, duration, stdout));
    }

    Ok(Metadata {
        width,
        height,
        duration,
    })
}
