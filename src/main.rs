use clap::{Parser, Subcommand};
use youtube::download_video;
use rutube::upload_to_rutube;
use telegram::upload_to_telegram;
use vk::upload_to_vk;
use transform::transform_video;
use anyhow::{Result, Context}; // Import Context from anyhow for better error handling

mod youtube;
mod rutube;
mod telegram;
mod vk;
mod transform;

#[derive(Parser)]
#[command(name = "youtube-to-platforms")]
#[command(about = "CLI tool to download YouTube videos and upload them to Rutube, Telegram, and VK")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Download {
        #[arg(short, long)]
        url: String,
        #[arg(short, long, default_value = "./videos")]
        output: String,
    },
    Transform {
        #[arg(short, long)]
        file: String,
    },
    Upload {
        #[arg(short, long)]
        platform: String,
        #[arg(short, long)]
        file: String,
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        api_key: Option<String>,
        #[arg(long, default_value = "https://api.telegram.org/")]
        bot_api_url: String,
        #[arg(long, default_value = "50000000")]
        max_file_size: u64,
        #[arg(short, long)]
        bot_token: Option<String>,
        #[arg(short, long)]
        chat_id: Option<i64>,
        #[arg(short, long)]
        vk_access_token: Option<String>,
    },
    Process {
        #[arg(short, long)]
        url: String,
        #[arg(short, long)]
        platform: String,
        #[arg(short, long, default_value = "./videos")]
        output: String,
        #[arg(short, long)]
        api_key: Option<String>,
        #[arg(long, default_value = "https://api.telegram.org/")]
        bot_api_url: String,
        #[arg(long, default_value = "50000000")]
        max_file_size: u64,
        #[arg(short, long)]
        bot_token: Option<String>,
        #[arg(short, long)]
        chat_id: Option<i64>,
        #[arg(short, long)]
        vk_access_token: Option<String>,
    },
}

#[tokio::main] // This makes the main function asynchronous
async fn main() -> Result<()> { // Use anyhow::Result
    let cli = Cli::parse();

    match cli.command {
        Commands::Download { url, output } => {
            println!("Downloading from: {}", url);
            println!("Saving to: {}", output);
            download_video(&url, &output)?;
        }
        Commands::Transform { file } => {
            let transformed_file = transform_video(&file)?;
            println!("Transformed video saved as: {}", transformed_file);
        }
        Commands::Upload {
            platform,
            file,
            title,
            api_key,
            bot_api_url,
            max_file_size,
            bot_token,
            chat_id,
            vk_access_token,
        } => {
            upload(platform, &file, &title, api_key, &bot_api_url, max_file_size,
                   bot_token, chat_id, vk_access_token).await?;
        }
        Commands::Process {
            url,
            platform,
            output,
            api_key,
            bot_api_url,
            max_file_size,
            bot_token,
            chat_id,
            vk_access_token,
        } => {
            println!("Starting process: Download -> Transform -> Upload");

            // Шаг 1: Загрузка видео
            println!("Downloading from: {}", url);
            let (downloaded_file, title) = download_video(&url, &output)?;
            println!("Downloaded file: {:?}", downloaded_file);

            // Шаг 2: Трансформация видео
            println!("Transforming video: {}", downloaded_file);
            let transformed_file = transform_video(&downloaded_file)?;
            println!("Transformed video saved as: {}", transformed_file);

            upload(platform, &transformed_file, &title, api_key, &bot_api_url,
                   max_file_size, bot_token, chat_id, vk_access_token).await?;
        }
        _ => {}
    }

    Ok(())
}

async fn upload(platform: String, file: &String, title: &String, api_key: Option<String>,
                bot_api_url: &String, max_file_size: u64, bot_token: Option<String>,
                chat_id: Option<i64>, vk_access_token: Option<String>) -> Result<()> {
    match platform.as_str() {
        "rutube" => {
            if let Some(key) = api_key {
                println!("Uploading '{}' to Rutube", file);
                upload_to_rutube(&key, &file, &title)?;
            } else {
                println!("API key for Rutube is missing.");
            }
        }
        "telegram" => {
            if let (Some(token), Some(id)) = (bot_token, chat_id) {
                println!("Uploading '{}' to Telegram", file);
                // Await the asynchronous upload function
                upload_to_telegram(
                    &bot_api_url.as_str(), max_file_size, &token, id, &file, &title)
                    .await.context("Failed to upload video to Telegram")?;
            } else {
                println!("Bot token or chat ID for Telegram is missing.");
            }
        }
        "vk" => {
            if let Some(token) = vk_access_token {
                println!("Uploading '{}' to VK", file);
                upload_to_vk(&token, &title, &file)?;
            } else {
                println!("VK access token is missing.");
            }
        }
        _ => println!("Unsupported platform: {}", platform),
    }
    Ok(())
}