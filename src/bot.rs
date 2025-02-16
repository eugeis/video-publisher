use teloxide::{net, Bot, prelude::*, types::Message, utils::command::BotCommands};
use teloxide::dispatching::{Dispatcher, UpdateFilterExt};
use teloxide::dptree::entry;
use teloxide::requests::Requester;

#[derive(BotCommands, Debug)]
#[command(rename_rule = "lowercase", description = "These are the possible commands:")]
enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Display help message")]
    Help,
    #[command(description = "A simple echo command")]
    Echo(String),
    #[command(description = "A command with no arguments")]
    NoArgs,
}

pub(crate) async fn run(        bot_token: String,
                                platform: String,
                                output: String,
                                api_key: Option<String>,
                                bot_api_url: String,
                                max_file_size: u64,
                                chat_id: Option<i64>,
                                vk_access_token: Option<String>,) -> anyhow::Result<()> {
    let client = net::default_reqwest_settings()
        .timeout(std::time::Duration::from_secs(240))
        .build()
        .expect("Client creation failed");

    let bot = Bot::with_client(bot_token, client);

    async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
        if let Some(text) = msg.text() {
            match Command::parse(text, "my_bot") { // Укажи реальное имя бота
                Ok(command) => match command {
                    Command::Start => bot.send_message(msg.chat.id, "Welcome!").await?,
                    Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
                    Command::Echo(arg) => bot.send_message(msg.chat.id, format!("You said: {}", arg)).await?,
                    Command::NoArgs => bot.send_message(msg.chat.id, "This command has no arguments.").await?,
                },
                Err(_) => bot.send_message(msg.chat.id, "Command not recognized. Try /help.").await?,
            };
        }
        Ok(())
    }

    let handler = entry()
        .branch(Update::filter_message().endpoint(handle_message));

    Dispatcher::builder(bot, handler).build().dispatch().await;

    Ok(())
}
