use color_eyre::eyre::Result;
use homeworkbot::{telegram::Telegram, Messenger};
use std::env;

fn main() -> Result<()> {
    color_eyre::install()?;

    let token = env::var("BOT_TOKEN").expect("BOT_TOKEN environment variable");

    let mut bot = Telegram::new(&token)?;
    println!("Started the bot");

    loop {
        let action = bot.get_updates()?;
        dbg!(action);
    }
}
