use clap::{Parser, Subcommand};
use youtube::download_video;
use rutube::upload_to_rutube;
use telegram::upload_to_telegram;
use vk::upload_to_vk;
use anyhow::{Result, Context}; // Import Context from anyhow for better error handling

mod youtube;
mod rutube;
mod telegram;
mod vk;

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
    Upload {
        #[arg(short, long)]
        platform: String,
        #[arg(short, long)]
        file: String,
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        description: String,
        #[arg(short, long)]
        api_key: Option<String>,      // Optional access key for Rutube
        #[arg(short, long)]
        bot_token: Option<String>,    // Optional bot token for Telegram
        #[arg(short, long)]
        chat_id: Option<i64>,         // Optional chat ID for Telegram
        #[arg(short, long)]
        vk_access_token: Option<String>, // Optional access token for VK
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
        Commands::Upload {
            platform,
            file,
            title,
            description,
            api_key,
            bot_token,
            chat_id,
            vk_access_token,
        } => {
            match platform.as_str() {
                "rutube" => {
                    if let Some(key) = api_key {
                        println!("Uploading '{}' to Rutube", file);
                        upload_to_rutube(&key, &file, &title, &description)?;
                    } else {
                        println!("API key for Rutube is missing.");
                    }
                }
                "telegram" => {
                    if let (Some(token), Some(id)) = (bot_token, chat_id) {
                        println!("Uploading '{}' to Telegram", file);
                        // Await the asynchronous upload function
                        upload_to_telegram(&token, id, &file, &description).await.context("Failed to upload video to Telegram")?;
                    } else {
                        println!("Bot token or chat ID for Telegram is missing.");
                    }
                }
                "vk" => {
                    if let Some(token) = vk_access_token {
                        println!("Uploading '{}' to VK", file);
                        upload_to_vk(&token, &title, &description, &file)?;
                    } else {
                        println!("VK access token is missing.");
                    }
                }
                _ => println!("Unsupported platform: {}", platform),
            }
        }
    }

    Ok(())
}