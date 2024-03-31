use color_eyre::eyre::Result;
use frankenstein::{
    AsyncApi, AsyncTelegramApi, GetUpdatesParams, Message, SendMessageParams, Update, UpdateContent,
};
use homeworkbot::{
    conversation::{self, NLPError},
    App,
};
use std::{env, fs};
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let (api, state_path) = setup()?;
    info!("Hello, world!");

    let mut cache = GetUpdatesParams::builder().build();
    let mut state = match restore_state(&state_path) {
        Ok(state) => state,
        Err(_) => App::new(),
    };

    loop {
        let response = api.get_updates(&cache).await?;

        process_updates(&api, &mut state, &state_path, &response.result).await?;

        if let Some(update) = response.result.last() {
            cache = GetUpdatesParams::builder()
                .offset(update.update_id + 1)
                .build();
        }

        debug!(?state);
    }
}

async fn process_updates(
    api: &AsyncApi,
    state: &mut App,
    state_path: &str,
    updates: &Vec<Update>,
) -> Result<()> {
    for update in updates {
        if let UpdateContent::Message(message) = &update.content {
            let api = api.clone();
            process_message(api, state, state_path, message).await?;
        }
    }

    Ok(())
}

async fn process_message(
    api: AsyncApi,
    state: &mut App,
    state_path: &str,
    message: &Message,
) -> Result<()> {
    let chat_id = message.chat.id;

    if let Some(message) = &message.text {
        info!(%chat_id, %message, "received:");

        match conversation::process_message(chat_id, String::from(message), state) {
            Ok(response) => {
                info!(?response, "replied: ");
                for text in response {
                    send_message(&api, chat_id, &text).await?;

                    if &text == "ok" {
                        save_state(state_path, state)?;
                        info!("saved state");
                    }
                }
            }
            Err(NLPError::NoPermission) => {
                info!("permission denied");
                send_message(&api, chat_id, "permission denied").await?;
            }
            Err(NLPError::InvalidCommand) => {
                info!("asked for a valid command");
                send_message(&api, chat_id, "invalid command").await?;
            }
            Err(NLPError::InvalidWeekday) => {
                info!("asked for a valid weekday");
                send_message(&api, chat_id, "not a valid weekday").await?;
            }
            Err(NLPError::ChatNotFound) => {
                error!("Chat #{chat_id} not found, message: {message}");
            }
            Err(NLPError::ParseError) => {
                error!("Parse error while processing {message}");
            }
            Err(NLPError::NothingToDo) => info!("nothing to do"),
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

fn save_state(path: &str, state: &App) -> Result<()> {
    let state = ron::to_string(state)?;

    fs::write(path, state)?;

    Ok(())
}

fn restore_state(path: &str) -> Result<App> {
    let state = fs::read_to_string(path)?;

    let state = ron::from_str(&state)?;

    Ok(state)
}

fn setup() -> Result<(AsyncApi, String)> {
    color_eyre::install()?;

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let token = env::var("BOT_TOKEN").expect("The BOT_TOKEN environment variable was not set.");
    let path = env::var("STATE_PATH").expect("The STATE_PATH environment variable was not set.");

    Ok((AsyncApi::new(&token), path))
}
