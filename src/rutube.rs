use reqwest::blocking::Client;
use serde_json::json;
use std::fs::File;
use std::io::Read;
use anyhow::Result;

pub fn upload_to_rutube(api_key: &str, file_path: &str, title: &str, description: &str) -> Result<()> {
    let client = Client::new();
    
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let form = reqwest::blocking::multipart::Form::new()
        .file("video", file_path)?;

    let res = client.post("https://rutube.ru/api/video/upload/")
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form)
        .send()?;

    println!("Upload response: {:?}", res.text()?);
    Ok(())
}
