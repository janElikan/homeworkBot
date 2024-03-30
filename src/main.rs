#![allow(unused)]
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use color_eyre::eyre::{OptionExt, Result};
use frankenstein::{
    AsyncApi, AsyncTelegramApi, GetUpdatesParams, Message, SendMessageParams, Update, UpdateContent,
};
use std::{collections::HashMap, env};

#[derive(Debug)]
struct App {
    assignments: HashMap<String, Assignment>,
    users: HashMap<u64, User>,
    chats: HashMap<i64, Vec<String>>,
    schedule: HashMap<Weekday, Day>,
}

#[derive(Debug)]
struct Assignment {
    text: String,
    attachments: Vec<String>, // of UUIDs
}

#[derive(Debug)]
struct Day {
    timetable: Vec<(NaiveTime, NaiveTime)>,
    periods: Vec<Option<String>>,
}

#[derive(Debug)]
struct User {
    first_name: String,
    last_name: Option<String>,
    role: Role,
}

#[derive(Debug)]
enum Role {
    Banned,
    User,
    Admin,
}

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
    let chat = message.chat.id;

    if let Some(message) = &message.text {
        let message = message.as_str();

        if message.starts_with('/') {
            let mut split = message.split(' ');
            let command = split.next().ok_or_eyre("expected to find a command")?;
            state.reset_history(chat);

            match command {
                "/get" => state.push_history(chat, String::from(command)),
                "/set" => {
                    let subject = split.next();
                    let task: String = split.collect();

                    if subject.is_none() {
                        state.push_history(chat, String::from(command));
                    } else if task.is_empty() {
                        let subject = subject
                            .ok_or_eyre("expected to find a subject")?
                            .to_string();

                        state.push_history(chat, String::from(command));
                        state.push_history(chat, subject);
                    } else {
                        let subject = subject
                            .ok_or_eyre("expected to find a subject")?
                            .to_string();

                        state.push_history(chat, String::from(command));
                        state.push_history(chat, subject);
                        state.push_history(chat, task);
                    }
                }
                _ => (),
            };
        } else {
            state.push_history(chat, String::from(message));
        };

        let Some(history) = state.history(chat) else {
            return send_message(&api, chat, "please enter a command").await;
        };
        let mut history = history.iter();

        let Some(command) = history.next() else {
            println!("no command");
            return Ok(());
        };
        let command = command.as_str();

        // TODO find a crate that does that with enums
        let response = match command {
            "/get" => state.get(Local::now()),
            "/set" => {
                let subject = history.next();
                let text: String = history.map(String::from).collect();

                if subject.is_none() {
                    wrap_message("what's the subject?")
                } else if text.is_empty() {
                    wrap_message("what's the assignment?")
                } else {
                    let subject = subject
                        .ok_or_eyre("expected to find a subject")?
                        .to_string();

                    state.set(
                        subject,
                        Assignment {
                            text,
                            attachments: Vec::new(),
                        },
                    );

                    wrap_message("ok")
                }
            }
            _ => {
                println!("no command");
                return Ok(());
            }
        };

        for text in response {
            send_message(&api, chat, &text).await?;
        }
    }

    Ok(())
}

// remove this later
fn wrap_message(text: &str) -> Vec<String> {
    vec![String::from(text)]
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

impl App {
    pub fn new() -> Self {
        Self {
            assignments: HashMap::new(),
            users: HashMap::new(),
            chats: HashMap::new(),
            schedule: HashMap::new(),
        }
    }

    pub fn get(&self, due: DateTime<Local>) -> Vec<String> {
        // TODO implement due lookup
        self.assignments
            .iter()
            .map(|(subject, assignment)| format!("{}: {}", subject, assignment.text))
            .collect()
    }

    pub fn set(&mut self, subject: String, assignment: Assignment) {
        self.assignments.insert(subject, assignment);
    }

    pub fn history(&self, chat: i64) -> Option<&Vec<String>> {
        self.chats.get(&chat)
    }

    pub fn push_history(&mut self, chat: i64, message: String) {
        match self.chats.get_mut(&chat) {
            None => {
                self.chats.insert(chat, Vec::new());
            }
            Some(chat) => chat.push(message),
        };
    }

    pub fn reset_history(&mut self, chat: i64) {
        match self.chats.get_mut(&chat) {
            None => {
                self.chats.insert(chat, Vec::new());
            }
            Some(chat) => chat.clear(),
        };
    }
}
