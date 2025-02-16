use clap::{Parser, Subcommand};
use youtube::download_video;
use transform::transform_video;
use anyhow::{Result, Context};
use teloxide::requests::Requester;

mod youtube;
mod rutube;
mod telegram;
mod vk;
mod transform;
mod bot;
mod upload;

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
    Bot {
        #[arg(short, long)]
        bot_token: String,
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
            upload::upload(platform, &file, &title, api_key, &bot_api_url, max_file_size,
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

            upload::upload(platform, &transformed_file, &title, api_key, &bot_api_url,
                   max_file_size, bot_token, chat_id, vk_access_token).await?;
        }
        Commands::Bot {
            bot_token,
            platform,
            output,
            api_key,
            bot_api_url,
            max_file_size,
            chat_id,
            vk_access_token,
        } => {
            println!("Starting Telegram bot...");
            bot::run(bot_token,
                     platform,
                     output,
                     api_key,
                     bot_api_url,
                     max_file_size,
                     chat_id,
                     vk_access_token).await?;
        }
    }

    Ok(())
}