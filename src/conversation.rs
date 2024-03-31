use super::{App, Assignment, Command};
use chrono::{Days, Local, NaiveTime, Weekday};
use tracing::info;

/// natural language processing error
pub enum NLPError {
    InvalidCommand,
    InvalidWeekday,
    ChatNotFound,
    NothingToDo,
    ParseError,
}

/// Compares a given message against the app's state, updates the state and returns a reply
///
/// # Errors
///
/// - `InvalidCommand` if the `message` starts with "/" but is not in the `homeworkbot::Command`
/// - `InvalidWeekday` if a weekday is expected but something else is given
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
            Command::Get | Command::GetAll => state.push_cmd(chat_id, command),
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
            Command::Delete => {
                let subject = split.next();

                if subject.is_none() {
                    state.push_cmd(chat_id, command);
                } else {
                    let subject = subject.ok_or(NLPError::ParseError)?.to_string();

                    state.push_cmd(chat_id, command);
                    state.push_arg(chat_id, subject);
                }
            }
            Command::SetSchedule => {
                let weekday = split.next();
                let periods: String = split.collect();

                state.push_cmd(chat_id, command);
                if let Some(weekday) = weekday {
                    parse_weekday(weekday)?; // to check if it's valid
                    state.push_arg(chat_id, String::from(weekday));
                }

                if !periods.is_empty() {
                    state.push_arg(chat_id, periods);
                }
            }
        };
    } else {
        state.push_arg(chat_id, message);
    };

    // what a hacky workaround :/
    // going to fix after alpha
    let chat = state
        .chats
        .get_mut(&chat_id)
        .ok_or(NLPError::ChatNotFound)?;

    let mut args = chat.args.iter();

    let command = chat.command.clone().ok_or(NLPError::NothingToDo)?;

    let response = match command {
        Command::Get => {
            let now = Local::now();

            let due =
                if now.time() > NaiveTime::from_hms_opt(12, 0, 0).ok_or(NLPError::ParseError)? {
                    now + Days::new(1)
                } else {
                    now
                };

            info!(%due);

            chat.clear();

            let assignments = state.get(due);
            if assignments.is_empty() {
                wrap_message("there's nothing")
            } else {
                assignments
            }
        }
        Command::GetAll => {
            chat.clear();
            state.get_all()
        }
        Command::Set => {
            let subject = args.next();
            let text: String = args.map(String::from).collect();

            if subject.is_none() {
                wrap_message("what's the subject?")
            } else if text.is_empty() {
                wrap_message("what's the assignment?")
            } else {
                let subject = subject.ok_or(NLPError::ParseError)?.to_string();

                chat.clear();
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
        Command::Delete => {
            let subject = args.next();

            if subject.is_none() {
                wrap_message("what's the subject?")
            } else {
                let subject = subject.ok_or(NLPError::ParseError)?.to_string();

                chat.clear();
                state.delete(subject);

                wrap_message("ok")
            }
        }
        Command::SetSchedule => {
            let weekday = args.next();
            let subjects = args.next();

            if weekday.is_none() {
                wrap_message("for what weekday?")
            } else if subjects.is_none() {
                if let Err(err) = parse_weekday(weekday.ok_or(NLPError::ParseError)?) {
                    // at least it works
                    // for now...
                    chat.args.pop();
                    return Err(err);
                }

                vec![
                    "list the subjects (comma-separated)".to_string(),
                    "example:".to_string(),
                    "english, science, cs".to_string(),
                    "use 'none' if some are missing, example:".to_string(),
                    "none, english, none, science".to_string(),
                ]
            } else {
                let weekday = parse_weekday(weekday.ok_or(NLPError::ParseError)?)?;
                let schedule: Vec<_> = subjects
                    .ok_or(NLPError::ParseError)?
                    .split(',')
                    .map(str::trim)
                    .map(|subject| {
                        if subject.to_lowercase() == "none" {
                            None
                        } else {
                            Some(subject.to_string())
                        }
                    })
                    .collect();

                chat.clear();
                state.set_schedule(weekday, schedule);

                wrap_message("ok")
            }
        }
    };

    Ok(response)
}

fn wrap_message(text: &str) -> Vec<String> {
    vec![String::from(text)]
}

fn parse_weekday(day: &str) -> Result<Weekday, NLPError> {
    day.parse().map_err(|_| NLPError::InvalidWeekday)
}
