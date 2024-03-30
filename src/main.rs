#![allow(unused)]
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use color_eyre::eyre::{OptionExt, Result};
use frankenstein::{
    AsyncApi, AsyncTelegramApi, GetUpdatesParams, Message, SendMessageParams, Update, UpdateContent,
};
use std::{collections::HashMap, env, str::FromStr};
use strum::EnumString;

#[derive(Debug)]
struct App {
    assignments: HashMap<String, Assignment>,
    users: HashMap<u64, User>,
    chats: HashMap<i64, Chat>,
    schedule: HashMap<Weekday, Day>,
}

#[derive(Debug)]
struct Chat {
    command: Option<Command>,
    args: Vec<String>,
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

#[derive(Debug, EnumString)]
#[strum(ascii_case_insensitive)]
enum Command {
    Get,
    Set,
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
    let chat_id = message.chat.id;

    if let Some(message) = &message.text {
        let message = message.as_str();

        if let Some(message) = message.strip_prefix('/') {
            let mut split = message.split(' ');
            let command = split.next().ok_or_eyre("expected to find a command")?;
            let command: Command = match command.parse() {
                Ok(cmd) => cmd,
                Err(_) => return send_message(&api, chat_id, "invalid command").await,
            };
            state.reset_chat(chat_id);

            match command {
                Command::Get => state.push_cmd(chat_id, command),
                Command::Set => {
                    let subject = split.next();
                    let task: String = split.collect();

                    if subject.is_none() {
                        state.push_cmd(chat_id, command);
                    } else if task.is_empty() {
                        let subject = subject
                            .ok_or_eyre("expected to find a subject")?
                            .to_string();

                        state.push_cmd(chat_id, command);
                        state.push_arg(chat_id, subject);
                    } else {
                        let subject = subject
                            .ok_or_eyre("expected to find a subject")?
                            .to_string();

                        state.push_cmd(chat_id, command);
                        state.push_arg(chat_id, subject);
                        state.push_arg(chat_id, task);
                    }
                }
            };
        } else {
            state.push_arg(chat_id, String::from(message));
        };

        let Some(chat) = state.get_chat(chat_id) else {
            return send_message(&api, chat_id, "please enter a command").await;
        };
        let mut args = chat.args.iter();

        let Some(command) = &chat.command else { return Ok(()); };

        let response = match command {
            Command::Get => state.get(Local::now()),
            Command::Set => {
                let subject = args.next();
                let text: String = args.map(String::from).collect();

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
        };

        for text in response {
            send_message(&api, chat_id, &text).await?;
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

    pub fn get_chat(&self, id: i64) -> Option<&Chat> {
        self.chats.get(&id)
    }

    pub fn push_cmd(&mut self, chat: i64, command: Command) {
        match self.chats.get_mut(&chat) {
            None => {
                self.chats.insert(chat, Chat {command: Some(command), args: Vec::new()});
            }
            Some(chat) => chat.command = Some(command),
        };
    }

    pub fn push_arg(&mut self, chat: i64, arg: String) {
        match self.chats.get_mut(&chat) {
            None => {
                self.chats.insert(chat, Chat {command: None, args: Vec::new()});
            }
            Some(chat) => chat.args.push(arg),
        };
    }

    pub fn reset_chat(&mut self, chat: i64) {
        match self.chats.get_mut(&chat) {
            None => {
                self.chats.insert(chat, Chat::new());
            }
            Some(chat) => chat.clear(),
        };
    }
}

impl Chat {
    pub const fn new() -> Self {
        Self { command: None, args: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.command = None;
        self.args.clear();
    }
}
