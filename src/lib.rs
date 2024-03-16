#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;

use chrono::{NaiveDate, NaiveTime, Weekday};
use color_eyre::eyre::Result;

pub trait State {
    fn get_chat_state(&self, user_id: String) -> Result<Vec<String>>;
    fn add_message(&mut self, user_id: String, message: String) -> Result<()>;
    fn reset_chat_state(&mut self, user_id: String) -> Result<()>;

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
    fn fetch(&self) -> Result<()>;
    fn last(&self) -> Result<Option<String>>;
    fn reply(&self, message: String) -> Result<()>;
}

pub mod storage;
pub mod telegram;

pub fn run(messenger: &mut dyn Messenger, state: &mut dyn State) -> Result<()> {
    todo!();
}
