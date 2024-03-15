use std::collections::HashMap;

use chrono::NaiveDate;

use crate::{Period, Permissions, State};

pub struct Storage {
    chat_state: Vec<Chat>,
    users: Vec<User>,
    assignments: HashMap<String, String>,
    schedule: Vec<Period>,
    schedule_overwrite: Option<ScheduleOverwrite>,
}

struct Chat {
    user_id: String,
    messages: Vec<String>,
}

struct User {
    id: String,
    permissions: Permissions,
}

struct ScheduleOverwrite {
    date: NaiveDate,
    schedule: Vec<Period>,
}

impl Storage {
    #[must_use]
    pub fn new() -> Self {
        Self {
            chat_state: Vec::new(),
            users: Vec::new(),
            assignments: HashMap::new(),
            schedule: Vec::new(),
            schedule_overwrite: None,
        }
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

impl State for Storage {
    fn get_chat_state(&self, user_id: String) -> color_eyre::eyre::Result<Vec<String>> {
        todo!()
    }
    fn add_message(&self, user_id: String) -> color_eyre::eyre::Result<()> {
        todo!()
    }
    fn reset_chat_state(&self, user_id: String) -> color_eyre::eyre::Result<()> {
        todo!()
    }

    fn get_permissions(&self, user_id: String) -> color_eyre::eyre::Result<Permissions> {
        todo!()
    }
    fn set_permissions(
        &self,
        user_id: String,
        permissions: Permissions,
    ) -> color_eyre::eyre::Result<()> {
        todo!()
    }

    fn get_assignments(
        &self,
        subject: Option<String>,
        due: Option<chrono::prelude::NaiveTime>,
    ) -> color_eyre::eyre::Result<Vec<String>> {
        todo!()
    }
    fn set_assignments(
        &self,
        subject: Option<String>,
        assignment: String,
    ) -> color_eyre::eyre::Result<()> {
        todo!()
    }

    fn get_schedule(&self, periods: Vec<Period>) -> color_eyre::eyre::Result<Period> {
        todo!()
    }
    fn set_schedule(&self, periods: Vec<Period>) -> color_eyre::eyre::Result<()> {
        todo!()
    }
    fn overwrite_schedule(
        &self,
        date: NaiveDate,
        periods: Vec<Period>,
    ) -> color_eyre::eyre::Result<()> {
        todo!()
    }
}
