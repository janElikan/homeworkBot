use color_eyre::eyre::Result;
use std::env;

use frankenstein::{AsyncApi, AsyncTelegramApi};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let token = env::var("BOT_TOKEN").expect("BOT_TOKEN environment variable");

    let api = AsyncApi::new(&token);
    match api.get_me().await {
        Ok(response) => println!(
            "Got a username: {}",
            response
                .result
                .username
                .expect("Accessed the API but failed to get a valid username")
        ),
        Err(error) => panic!("Failed to get a username: {:#?}", error),
    }

    Ok(())
}
