use color_eyre::eyre::Result;
use homeworkbot::App;
use std::env;

use frankenstein::{AllowedUpdate, AsyncApi, AsyncTelegramApi, GetUpdatesParams, Message, SendMessageParams, UpdateContent};

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
        Err(error) => panic!("Failed to get a username: {error:#?}"),
    }

    let update_params_builder = GetUpdatesParams::builder();

    let mut update_params = update_params_builder
        .clone()
        .allowed_updates(vec![AllowedUpdate::Message])
        .build();

    loop {
        let updates = match api.get_updates(&update_params).await {
            Ok(response) => response.result,
            Err(err) => {
                eprint!("Failed to get updates: {err:#?}");
                continue;
            }
        };

        for update in updates {
            if let UpdateContent::Message(message) = update.content {
                let cloned_api = api.clone();

                tokio::spawn(async move {
                    match process_message(message, cloned_api).await {
                        Ok(()) => println!("processed message"),
                        Err(error) => eprintln!("failed processing message: {error:#?}"),
                    };
                });
            };

            update_params = update_params_builder
                    .clone()
                    .offset(update.update_id + 1)
                    .build();
        }
    }
}

async fn process_message(message: Message, api: AsyncApi) -> Result<()> {
    dbg!(&message);
    let response = "hi. I have no logic yet, just the scaffolding";

    let params = SendMessageParams::builder()
        .chat_id(message.chat.id)
        .text(response)
        .build();

    match api.send_message(&params).await {
        Ok(_) => Ok(()),
        Err(error) => Err(error.into()),
    }
}

enum ActiveCommand {
    None,
    // Assignments
    Set,
    Delete,

    // Subjects
    DeleteSubject,

    // Admins
    Promote,
    Demote,

    // Schedule
    SetSchedule,
    UpdateTomorrowSchedule,
}
