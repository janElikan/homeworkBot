#![allow(unused)]
use chrono::{DateTime, Local, NaiveTime, Weekday};
use std::collections::HashMap;
use strum::EnumString;

#[derive(Debug)]
pub struct App {
    pub assignments: HashMap<String, Assignment>,
    pub users: HashMap<u64, User>,
    pub chats: HashMap<i64, Chat>,
    pub schedule: HashMap<Weekday, Day>,
}

#[derive(Debug)]
pub struct Chat {
    pub command: Option<Command>,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub struct Assignment {
    pub text: String,
    pub attachments: Vec<String>, // of UUIDs
}

#[derive(Debug)]
pub struct Day {
    pub timetable: Vec<(NaiveTime, NaiveTime)>,
    pub periods: Vec<Option<String>>,
}

#[derive(Debug)]
pub struct User {
    pub first_name: String,
    pub last_name: Option<String>,
    pub role: Role,
}

#[derive(Debug)]
pub enum Role {
    Banned,
    User,
    Admin,
}

#[derive(Debug, Clone, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Command {
    Get,
    Set,
}

pub mod conversation;

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    #[must_use]
    pub fn new() -> Self {
        Self {
            assignments: HashMap::new(),
            users: HashMap::new(),
            chats: HashMap::new(),
            schedule: HashMap::new(),
        }
    }

    #[must_use]
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

    #[must_use]
    pub fn get_chat(&self, id: i64) -> Option<&Chat> {
        self.chats.get(&id)
    }

    pub fn push_cmd(&mut self, chat: i64, command: Command) {
        match self.chats.get_mut(&chat) {
            None => {
                self.chats.insert(
                    chat,
                    Chat {
                        command: Some(command),
                        args: Vec::new(),
                    },
                );
            }
            Some(chat) => chat.command = Some(command),
        };
    }

    pub fn push_arg(&mut self, chat: i64, arg: String) {
        match self.chats.get_mut(&chat) {
            None => {
                self.chats.insert(
                    chat,
                    Chat {
                        command: None,
                        args: Vec::new(),
                    },
                );
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
    #[must_use]
    pub const fn new() -> Self {
        Self {
            command: None,
            args: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.command = None;
        self.args.clear();
    }
}