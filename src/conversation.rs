use super::{App, Assignment, Command};
use chrono::Local;

/// natural language processing error
pub enum NLPError {
    InvalidCommand,
    ChatNotFound,
    NothingToDo,
    ParseError,
}

/// Compares a given message against the app's state, updates the state and returns a reply
///
/// # Errors
///
/// - `InvalidCommand` if the `message` starts with "/" but is not in the `homeworkbot::Command`
/// - `ChatNotFound` if `message` is not a command and `chat_id` is not in `state.chats`
/// - `NothingToDo` when there's no active command in the current chat and the user isn't calling a
/// new one
/// - `ParseError` is when something went wrong inside the function (I just don't want to .expect
/// anything for now)
pub fn process_message(
    chat_id: i64,
    message: String,
    state: &mut App,
) -> Result<Vec<String>, NLPError> {
    if let Some(message) = message.strip_prefix('/') {
        let mut split = message.split(' ');
        let command = split.next().ok_or(NLPError::ParseError)?;
        let command: Command = match command.parse() {
            Ok(cmd) => cmd,
            Err(_) => return Err(NLPError::InvalidCommand),
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
                    let subject = subject.ok_or(NLPError::ParseError)?.to_string();

                    state.push_cmd(chat_id, command);
                    state.push_arg(chat_id, subject);
                } else {
                    let subject = subject.ok_or(NLPError::ParseError)?.to_string();

                    state.push_cmd(chat_id, command);
                    state.push_arg(chat_id, subject);
                    state.push_arg(chat_id, task);
                }
            }
        };
    } else {
        state.push_arg(chat_id, message);
    };

    let Some(chat) = state.get_chat(chat_id) else {
        return Err(NLPError::ChatNotFound);
    };
    let mut args = chat.args.iter();

    let command = chat.command.clone().ok_or(NLPError::NothingToDo)?;

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
                let subject = subject.ok_or(NLPError::ParseError)?.to_string();

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

    Ok(response)
}

fn wrap_message(text: &str) -> Vec<String> {
    vec![String::from(text)]
}
