use reqwest::blocking::Client;
use serde_json::Value;
use anyhow::Result;
use std::fs::File;
use std::io::Read;

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
        .ok_or_else(|| anyhow::anyhow!("Failed to get upload URL"))?
        .to_string();

    // Step 2: Upload video
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let form = reqwest::blocking::multipart::Form::new()
        .file("video_file", file_path)?;

    let upload_res = client.post(&upload_url)
        .multipart(form)
        .send()?;

    println!("VK Upload Response: {:?}", upload_res.text()?);
    Ok(())
}
