use crate::Action;

// I didn't know what to call it
pub struct BotResponse<'a> {
    pub reply: &'a str,
    pub action: Option<Action>,
    pub chat_state: ChatState,
}

#[allow(dead_code)]
pub enum ChatState {
    Default,

    // Assignments
    Setting,
    Deleting,

    // Subjects
    DeletingSubject,

    // Admins
    Promoting,
    Demoting,

    // Schedule
    SettingSchedule,
    UpdatingTomorrowSchedule,
}

pub const fn process_message<'a>(_chat_state: &ChatState, _text: &str) -> BotResponse<'a> {
    BotResponse {
        reply: "unimplemented",
        action: None,
        chat_state: ChatState::Default,
    }
}
