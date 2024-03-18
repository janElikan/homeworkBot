use color_eyre::eyre::Result;
use homeworkbot::{storage::Storage, telegram::Telegram};
use std::env;

fn main() -> Result<()> {
    color_eyre::install()?;

    let token = env::var("BOT_TOKEN").expect("BOT_TOKEN environment variable");

    let mut bot = Telegram::new(&token)?;
    let mut store = Storage::new();

    println!("issued a token");

    loop {
        homeworkbot::run(&mut bot, &mut store).unwrap();
        dbg!(&store);
    }
}
