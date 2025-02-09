use serde_json::Value;
use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::{Read, Write};
use reqwest::blocking::{Client, multipart::Form}; // Import blocking Client and multipart::Form

pub fn upload_to_vk(access_token: &str, title: &str, description: &str, file_path: &str) -> Result<()> {
    let client = Client::new();

    // Step 1: Get upload URL
    let url = "https://api.vk.com/method/video.save";
    let res = client.get(url)
        .query(&[
            ("access_token", access_token),
            ("v", "5.131"),
            ("name", title),
            ("description", description),
        ])
        .send()?
        .json::<Value>()?;

    let upload_url = res["response"]["upload_url"].as_str()
        .ok_or_else(|| anyhow!("Failed to get upload URL"))?
        .to_string();

    // Step 2: Upload video
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Create a multipart form with the video file
    let form = Form::new()
        .file("video_file", file_path)?;

    // Step 3: Send video to the upload URL
    let upload_res = client.post(&upload_url)
        .multipart(form)
        .send()?;

    if !upload_res.status().is_success() {
        return Err(anyhow!("Failed to upload video to VK. Status: {}", upload_res.status()));
    }

    let upload_response_text = upload_res.text()?;
    println!("VK Upload Response: {}", upload_response_text);

    // Optionally, you can handle further steps like finalizing the upload on VK here.

    Ok(())
}
