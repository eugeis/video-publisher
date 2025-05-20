use std::fs;
use crate::transform::{transform_video, EncodingPasses};
use crate::upload;
use crate::youtube::download_video;

pub(crate) async fn youtube(
    url: &str, platform: &str, output: &str,
    delete_youtube: bool, delete_transformed: bool, rutube_api_key: Option<String>,
    bot_api_url: &str, max_file_size: u64, bot_token: Option<String>,
    chat_id: Option<i64>, vk_access_token: Option<String>) -> anyhow::Result<()> {

    println!("Starting process: Download -> Transform -> Upload");

    println!("Downloading from: {}", url);
    let (downloaded_file, title) = download_video(&url, &output)?;
    println!("Downloaded file: {:?}", downloaded_file);

    println!("Transforming video: {}", downloaded_file);
    let transformed_file = transform_video(&downloaded_file, EncodingPasses::TwoPass)?;
    println!("Transformed video saved as: {}", transformed_file);

    upload::upload(platform, &transformed_file, &title, rutube_api_key, &bot_api_url,
                   max_file_size, bot_token, chat_id, vk_access_token, url, "").await?;

    if delete_youtube {
        fs::remove_file(&downloaded_file)?;
    }
    if delete_transformed {
        fs::remove_file(&transformed_file)?;
    }
    Ok(())
}