use clap::{Parser, Subcommand};
use youtube::download_video;
use rutube::upload_to_rutube;
use telegram::upload_to_telegram;
use vk::upload_to_vk;
use anyhow::Result;

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
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Download { url, output } => {
            println!("Downloading from: {}", url);
            println!("Saving to: {}", output);
            download_video(&url, &output)?;
        }
        Commands::Upload { platform, file, title, description } => {
            match platform.as_str() {
                "rutube" => {
                    println!("Uploading '{}' to Rutube", file);
                    upload_to_rutube("your-api-key", &file, &title, &description)?;
                }
                "telegram" => {
                    println!("Uploading '{}' to Telegram", file);
                    upload_to_telegram("your-bot-token", "your-chat-id", &file, &description)?;
                }
                "vk" => {
                    println!("Uploading '{}' to VK", file);
                    upload_to_vk("your-vk-access-token", &title, &description, &file)?;
                }
                _ => println!("Unsupported platform: {}", platform),
            }
        }
    }

    Ok(())
}
