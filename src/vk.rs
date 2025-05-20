use serde_json::Value;
use anyhow::{Result, anyhow};
use reqwest::{Client, multipart::Form};

pub async fn upload_to_vk(access_token: &str, title: &str, file_path: &str) -> Result<()> {
    let client = Client::new();

    // Step 1: Get upload URL
    let url = "https://api.vk.com/method/video.save";
    let res = client.get(url)
        .query(&[
            ("access_token", access_token),
            ("v", "5.131"),
            ("name", title),
        ])
        .send()
        .await?
        .json::<Value>()
        .await?;

    let upload_url = res["response"]["upload_url"].as_str()
        .ok_or_else(|| anyhow!("Failed to get upload URL"))?
        .to_string();

    // Step 2: Upload video
    // Create a multipart form with the video file
    let form = Form::new()
        .file("video_file", file_path)?;

    // Step 3: Send video to the upload URL
    let upload_res = client.post(&upload_url)
        .multipart(form)
        .send()
        .await?;

    if !upload_res.status().is_success() {
        return Err(anyhow!("Failed to upload video to VK. Status: {}", upload_res.status()));
    }

    let upload_response_text = upload_res.text().await?;
    println!("VK Upload Response: {}", upload_response_text);

    // Optionally, you can handle further steps like finalizing the upload on VK here.

    Ok(())
}
