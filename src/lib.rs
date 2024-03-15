#![allow(dead_code)]
#![allow(unused_variables)]
use chrono::{NaiveDate, NaiveTime, Weekday};
use color_eyre::eyre::Result;

pub trait State {
    fn get_chat_state(&self, user_id: String) -> Result<Vec<String>>;
    fn add_message(&self, user_id: String) -> Result<()>;
    fn reset_chat_state(&self, user_id: String) -> Result<()>;

    fn get_permissions(&self, user_id: String) -> Result<Permissions>;
    fn set_permissions(&self, user_id: String, permissions: Permissions) -> Result<()>;

    /// empty subject means all
    /// empty due date means all pending
    fn get_assignments(
        &self,
        subject: Option<String>,
        due: Option<NaiveTime>,
    ) -> Result<Vec<String>>;
    fn set_assignments(&self, subject: Option<String>, assignment: String) -> Result<()>;

    fn get_schedule(&self, periods: Vec<Period>) -> Result<Period>;
    fn set_schedule(&self, periods: Vec<Period>) -> Result<()>;
    fn overwrite_schedule(&self, date: NaiveDate, periods: Vec<Period>) -> Result<()>;
}

#[derive(Debug)]
pub struct Period {
    pub weekday: Weekday,
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub name: String,
}

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
