#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;

use chrono::{NaiveDate, NaiveTime, Weekday};
use color_eyre::eyre::Result;

pub trait State {
    fn get_chat_state(&self, chat_id: String,user_id: String) -> Result<Vec<String>>;
    fn add_message(&mut self, chat_id: String,user_id: String, message: String) -> Result<()>;
    fn reset_chat_state(&mut self, chat_id: String, user_id: String) -> Result<()>;

    fn get_permissions(&self, user_id: String) -> Result<Permissions>;
    fn set_permissions(&mut self, user_id: String, permissions: Permissions) -> Result<()>;

    /// empty subject means all
    /// empty due date means all pending
    fn get_assignments(
        &self,
        subject: Option<String>,
        due: Option<NaiveTime>,
    ) -> Result<HashMap<String, String>>;
    fn set_assignment(&mut self, subject: String, assignment: Option<String>) -> Result<()>;

    fn get_schedule(&self, periods: Vec<Period>) -> Result<Vec<Period>>;
    fn set_schedule(&mut self, periods: Vec<Period>) -> Result<()>;
    fn overwrite_schedule(&mut self, date: NaiveDate, periods: Vec<Period>) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct Period {
    pub weekday: Weekday,
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Permissions {
    None,
    Read,
    ReadWrite,
}

pub trait Messenger {
    fn fetch(&mut self) -> Result<()>;
    fn first(&self) -> Option<MessengerUpdate>;
    fn reply(&mut self, message: String) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct MessengerUpdate {
    chat_id: String,
    user_id: String,
    content: String,
}

pub mod storage;
pub mod telegram;

pub fn run(messenger: &mut dyn Messenger, state: &mut dyn State) -> Result<()> {
    messenger.fetch()?; // todo!("should only call this when needed")
    let message = messenger.first();
    let MessengerUpdate{chat_id, user_id, content: message} = match message {
        Some(update) => update,
        None => return Ok(()),
    };
    if message.starts_with("/") {
        state.reset_chat_state(chat_id.clone(), user_id.clone())?;
    };
    state.add_message(chat_id, user_id, message)?;

    messenger.reply("unimplemented".to_string())?;
    Ok(())
}
