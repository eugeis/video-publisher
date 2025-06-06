use clap::{Parser, Subcommand, ValueEnum};
use youtube::download_video;
use transform::transform_video;
use anyhow::{Result};
use crate::transform::EncodingPasses;

mod youtube;
mod rutube;
mod telegram;
mod vk;
mod transform;
mod bot;
mod upload;
mod process;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Platform {
    Rutube,
    Telegram,
    Vk,
}

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
        platform: Platform,
        #[arg(short, long)]
        file: String,
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        rutube_api_key: Option<String>,
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
        platform: Platform,
        #[arg(short, long, default_value = "./videos")]
        output: String,
        #[arg(long)]
        delete_youtube: bool,
        #[arg(long)]
        delete_transformed: bool,
        #[arg(short, long)]
        rutube_api_key: Option<String>,
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
        platform: Platform,
        #[arg(short, long, default_value = "./videos")]
        output: String,
        #[arg(long)]
        delete_youtube: bool,
        #[arg(long)]
        delete_transformed: bool,
        #[arg(short, long)]
        rutube_api_key: Option<String>,
        #[arg(long, default_value = "https://api.telegram.org/")]
        bot_api_url: String,
        #[arg(long, default_value = "50000000")]
        max_file_size: u64,
        #[arg(short, long)]
        chat_id: Option<i64>,
        #[arg(short, long)]
        vk_access_token: Option<String>,
        #[arg(short, long, value_delimiter = ',')]
        allowed_users: Vec<u64>,
    },
}

impl Cli {
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Download { url, output } => {
            println!("Downloading from: {}", url);
            println!("Saving to: {}", output);
            download_video(&url, &output)?;
        }
        Commands::Transform { file } => {
            let transformed_file = transform_video(&file, EncodingPasses::TwoPass)?;
            println!("Transformed video saved as: {}", transformed_file);
        }
        Commands::Upload {
            platform,
            file,
            title,
            rutube_api_key,
            bot_api_url,
            max_file_size,
            bot_token,
            chat_id,
            vk_access_token,
        } => {
            upload::upload(platform, &file, &title, rutube_api_key, &bot_api_url, max_file_size,
                           bot_token, chat_id, vk_access_token, "", "").await?;
        }
        Commands::Process {
            url,
            platform,
            output,
            delete_youtube,
            delete_transformed,
            rutube_api_key,
            bot_api_url,
            max_file_size,
            bot_token,
            chat_id,
            vk_access_token,
        } => {
            process::youtube(&url, platform, &output, delete_youtube, delete_transformed,
                             rutube_api_key, &bot_api_url,
                             max_file_size, bot_token, chat_id, vk_access_token).await?;
        }
        Commands::Bot {
            bot_token,
            platform,
            output,
            delete_youtube,
            delete_transformed,
            rutube_api_key,
            bot_api_url,
            max_file_size,
            chat_id,
            vk_access_token,
            allowed_users,
        } => {
            println!("Telegram bot...");

            bot::run(&bot_token,
                     platform,
                     &output,
                     delete_youtube,
                     delete_transformed,
                     rutube_api_key,
                     &bot_api_url,
                     max_file_size,
                     chat_id,
                     vk_access_token,
                     allowed_users).await?;
        }
    }

    Ok(())
}
