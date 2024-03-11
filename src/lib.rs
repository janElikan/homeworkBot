use chrono::NaiveDate;
use color_eyre::eyre::Result;

mod conversation;
pub mod telegram;

#[allow(dead_code)]
pub struct App {
    pub current_assignments: Vec<String>,
    pub schedule: Vec<Vec<Period>>,
    pub overwrite_schedule: Option<Vec<Period>>,
    pub admins: Vec<String>,
}

#[derive(Debug)]
pub struct Period {
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub name: String,
}

#[derive(Debug)]
pub enum Action {
    SetAssignment((String, String)),
    DeleteAssignment(String),
    DeleteSubject(String),

    UpdateTomorrowSchedule(Vec<Period>),
    SetSchedule(Vec<Period>),

    PromoteUserId(String),
    DemoteUserId(String),
}

// todo: add error types
pub trait Messenger {
    fn get_updates(&mut self) -> Result<Option<Action>>;
}
