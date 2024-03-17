use std::collections::HashMap;

use chrono::NaiveDate;
use color_eyre::eyre::OptionExt;

use crate::{Period, Permissions, State};

pub struct Storage {
    chat_state: Vec<Chat>,
    users: Vec<User>,
    assignments: HashMap<String, String>,
    schedule: Vec<Period>,
    schedule_overwrite: Option<ScheduleOverwrite>,
}

struct Chat {
    id: String,
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
        let chat = self
            .chat_state
            .iter()
            .find(|chat| chat.user_id == user_id)
            .ok_or_eyre("chat not found")?;

        Ok(chat.messages.clone())
    }
    fn add_message(&mut self, user_id: String, message: String) -> color_eyre::eyre::Result<()> {
        let chat = self
            .chat_state
            .iter_mut()
            .find(|chat| chat.user_id == user_id)
            .ok_or_eyre("chat not found")?;
        chat.messages.push(message);

        Ok(())
    }
    fn reset_chat_state(&mut self, user_id: String) -> color_eyre::eyre::Result<()> {
        let chat = self
            .chat_state
            .iter_mut()
            .find(|chat| chat.user_id == user_id)
            .ok_or_eyre("chat not found")?;
        chat.messages.clear();

        Ok(())
    }

    fn get_permissions(&self, user_id: String) -> color_eyre::eyre::Result<Permissions> {
        let user = self
            .users
            .iter()
            .find(|user| user.id == user_id)
            .ok_or_eyre("user not found")?;

        Ok(user.permissions.clone())
    }
    fn set_permissions(
        &mut self,
        user_id: String,
        permissions: Permissions,
    ) -> color_eyre::eyre::Result<()> {
        let user = self
            .users
            .iter_mut()
            .find(|user| user.id == user_id)
            .ok_or_eyre("user not found")?;
        user.permissions = permissions;

        Ok(())
    }

    fn get_assignments(
        &self,
        subject: Option<String>,
        due: Option<chrono::prelude::NaiveTime>,
    ) -> color_eyre::eyre::Result<HashMap<String, String>> {
        Ok(self.assignments.clone())
    }
    fn set_assignment(
        &mut self,
        subject: String,
        assignment: Option<String>,
    ) -> color_eyre::eyre::Result<()> {
        match assignment {
            Some(assignment) => self.assignments.insert(subject, assignment),
            None => self.assignments.insert(subject, String::new()),
        };

        Ok(())
    }

    fn get_schedule(&self, periods: Vec<Period>) -> color_eyre::eyre::Result<Vec<Period>> {
        Ok(self.schedule.clone())
    }
    fn set_schedule(&mut self, periods: Vec<Period>) -> color_eyre::eyre::Result<()> {
        self.schedule = periods;

        Ok(())
    }
    fn overwrite_schedule(
        &mut self,
        date: NaiveDate,
        periods: Vec<Period>,
    ) -> color_eyre::eyre::Result<()> {
        self.schedule_overwrite = Some(ScheduleOverwrite {
            schedule: periods,
            date,
        });

        Ok(())
    }
}
