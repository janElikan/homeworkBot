use color_eyre::eyre::Result;
use frankenstein::{
    AsyncApi, AsyncTelegramApi, GetUpdatesParams, Message, SendMessageParams, Update, UpdateContent,
};
use homeworkbot::{
    conversation::{self, NLPError},
    App
};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let api = setup()?;

    let mut cache = GetUpdatesParams::builder().build();
    let mut state = App::new();

    loop {
        let response = api.get_updates(&cache).await?;

        process_updates(&api, &mut state, &response.result).await?;
        dbg!(&state);

        if let Some(update) = response.result.last() {
            cache = GetUpdatesParams::builder()
                .offset(update.update_id + 1)
                .build();
        }
    }
}

async fn process_updates(api: &AsyncApi, state: &mut App, updates: &Vec<Update>) -> Result<()> {
    for update in updates {
        if let UpdateContent::Message(message) = &update.content {
            let api = api.clone();
            process_message(api, state, message).await?;
        }
    }

    Ok(())
}

async fn process_message(api: AsyncApi, state: &mut App, message: &Message) -> Result<()> {
    let chat_id = message.chat.id;

    if let Some(message) = &message.text {
        match conversation::process_message(chat_id, String::from(message), state) {
            Ok(response) => {
                for text in response {
                    send_message(&api, chat_id, &text).await?;
                }
            }
            Err(NLPError::InvalidCommand) => {
                send_message(&api, chat_id, "invalid_command").await?;
            }
            Err(NLPError::ChatNotFound) => {
                eprintln!("Chat #{chat_id} not found, message: {message}");
            }
            Err(NLPError::ParseError) => {
                eprintln!("Parse error while processing {message}");
            }
            Err(NLPError::NothingToDo) => (),
        }
    }

    Ok(())
}

async fn send_message(api: &AsyncApi, chat: i64, text: &str) -> Result<()> {
    let message = SendMessageParams::builder()
        .chat_id(chat)
        .text(text)
        .build();

    api.send_message(&message).await?;

    Ok(())
}

fn setup() -> Result<AsyncApi> {
    color_eyre::install()?;

    let token = env::var("BOT_TOKEN").expect("The BOT_TOKEN environment variable was not set.");

    Ok(AsyncApi::new(&token))
}
