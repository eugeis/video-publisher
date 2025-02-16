use std::sync::Arc;
use regex::Regex;
use teloxide::{net, Bot, prelude::*, types::Message, utils::command::BotCommands};
use teloxide::dispatching::{Dispatcher, UpdateFilterExt};
use teloxide::dptree::entry;
use teloxide::requests::Requester;
use crate::process;

#[derive(BotCommands, Debug)]
#[command(rename_rule = "lowercase", description = "These are the possible commands:")]
enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Display help message")]
    Help,
}

// Shared configuration struct
#[derive(Clone)]
struct BotConfig {
    platform: String,
    output: String,
    rutube_api_key: Option<String>,
    bot_api_url: String,
    max_file_size: u64,
    bot_token: String,
    chat_id: Option<i64>,
    vk_access_token: Option<String>,
}

pub(crate) async fn run(bot_token: &str,
                        platform: &str,
                        output: &str,
                        rutube_api_key: Option<String>,
                        bot_api_url: &str,
                        max_file_size: u64,
                        chat_id: Option<i64>,
                        vk_access_token: Option<String>,) -> anyhow::Result<()> {
    let client = net::default_reqwest_settings()
        .timeout(std::time::Duration::from_secs(240))
        .build()
        .expect("Client creation failed");

    let bot = Bot::with_client(bot_token, client);

    let config = Arc::new(BotConfig {
        platform: platform.to_string(),
        output: output.to_string(),
        rutube_api_key,
        bot_api_url: bot_api_url.to_string(),
        max_file_size,
        bot_token: bot_token.to_string(),
        chat_id,
        vk_access_token,
    });

    async fn handle_message(bot: Bot, msg: Message, config: Arc<BotConfig>) -> ResponseResult<()> {
        let youtube_regex = Regex::new(r"^(https?://)?(www\.)?(youtube\.com/(watch\?v=|shorts/)|youtu\.be/)[\w-]+").unwrap();

        if let Some(text) = msg.text() {
            // Check if the message is a command
            if let Ok(command) = Command::parse(text, "my_bot") {
                match command {
                    Command::Start => {
                        bot.send_message(msg.chat.id, "Welcome!").await?;
                    }
                    Command::Help => {
                        bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
                    }
                }
            }
            // Check if the text is a YouTube URL
            else if youtube_regex.is_match(text) {
                bot.send_message(msg.chat.id, "Processing YouTube video...").await?;
                let cfg = config.clone();
                if let Err(e) = process::youtube(
                    text,
                    &cfg.platform,
                    &cfg.output,
                    cfg.rutube_api_key.clone(),
                    &cfg.bot_api_url,
                    cfg.max_file_size,
                    Some(cfg.bot_token.clone()),
                    cfg.chat_id,
                    cfg.vk_access_token.clone()
                ).await {
                    bot.send_message(msg.chat.id, format!("Error processing video: {}", e)).await?;
                } else {
                    bot.send_message(msg.chat.id, "Video processed successfully!").await?;
                }
            } else {
                bot.send_message(msg.chat.id, "Unknown command or message. Send a YouTube link or use /help.").await?;
            }
        }
        Ok(())
    }

    let handler = entry()
        .branch(Update::filter_message().endpoint({
            let bot_config = config.clone();
            move |bot, msg| handle_message(bot, msg, bot_config.clone())
        }));

    Dispatcher::builder(bot, handler).build().dispatch().await;

    Ok(())
}
