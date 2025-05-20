use anyhow::Result;
use reqwest::{Client, multipart::Form};

pub async fn upload_to_rutube(api_key: &str, file_path: &str, _title: &str) -> Result<()> {
    let client = Client::new();

    let form = Form::new()
        .file("video", file_path)?;

    let res = client
        .post("https://rutube.ru/api/video/upload/")
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form)
        .send()
        .await?;

    println!("Upload response: {:?}", res.text().await?);
    Ok(())
}
