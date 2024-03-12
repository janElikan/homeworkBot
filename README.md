> A homework assignment keeping telegram bot

Our class in high school has a problem with assignments. Teachers would constantly forget to put them into the system they were supposed to use and tell us to remember the assignments instead. No-one did, so they kept asking me. I grew tired of that, so I made this:

\<a video clip showing how it works />

Features:
- setting/getting assignments
- changing the schedule for tomorrow (happened a lot of times and was not supported by the system the school used)
- notifications when an admin added an assignment

## Progress
- [x] define the task
- [x] define types
- [x] create repo [[@pc]]
- [x] figure out how to use rust under NixOS
- [x] create README
- [x] create types [[@pc]]
- [x] create incoming message handling
- [ ] create conversation state [[@pc]]
- [x] define the telegram module [[@phone]]
- [x] move telegram things into a separate module
- [x] figure out how to actually fix the problem described below
- [ ] fix it
- [ ] implement schedules [[@pc]]
- [ ] implement assignments [[@pc]]
- [ ] implement admins [[@pc]]
- [ ] implement logging [[@pc]]
- [ ] improve error types [[@pc]]
- [ ] implement the storage [[@pc]]
- [ ] wrap builds in a flake [[@pc]]
- [ ] initial launch [[@pc]]
- [ ] research how to store files [[@phone]]
- [ ] add file uploads [[@pc]]
- [ ] add documentation [[@pc]]

## Server owner's manual (self-hosting)
The server is made as simple as possible, it's only capable of managing data for a single class. If you need to scale it to multiple, deploy multiple instances.

\<binary installation instructions here />

## Building from source
You'll need any machine that has the Nix package manager on it.

\<instructions on how to use it with nix here />

## Admin's manual
All the commands don't take any arguments and ask follow-up questions if needed

### Managing admins
- `/promote` then asks for you to send a contact. You can do that by going to that person's profile, tapping three dots in the top right and `share contact`.
- `/demote`

### Managing assignments
- `/set` gives you the list of subjects and an option to use the one that's currently in progress according to the bot's schedule, then asks for the assignment
- `/delete` presents you with a list of subjects

> [!NOTE] If an assignment for a certain subject is not changed, it's passed on to a future date

### Managing schedules
- `/update-tomorrow-schedule`, that's really meant for the teachers. It asks for a new list of subjects, you have to type their names manually.

## A peek under the hood
The app is designed to be as simple as possible, so it does not fully adhere to 12 factor app principles. They recommend storing persistent data in a database, but that would be quite overkill for my purposes. 12 factor apps are designed to scale, here you just host your own instance for each class.

I'm temporarily ditching async because I can't figure out how to work… Get back to that later.

\<excalidraw diagram here />

Modules:
- lib (common/types)
- telegram (Messenger implementation)
- file_store (Store implementation)
- conversation (Messenger's helper functions)

> [!ERROR] the current implementation can't process a message and then respond to it, one the action is created it's GONE
> one possible solution is to add a .reply(&str) method to BotResponse, maybe rename it to Command

### How solve that, exactly
To answer that, I'll first need to figure out the bot's architecture, I'm thinking:
- main runs `telegram.refresh()`
- main reads `telegram.last_update`
- and calls `conversation.process_command(update)`
- getting an `Action` as a result
- main calls `lib.process(action)`
- `lib.process` calls `store.load/save/whatever`
- `lib.process` returns a new `App` which I should probably rename to Data
- main calls `conversation.form_reply(data, last_action)` which returns a string
- main calls `telegram.reply(message)` which means the message is processed
- `telegram` deletes the last message from the queue replacing it with None

### Types
```rust
struct App {
  current_assignments: Vec<String>;
  schedule: Vec<Vec<Period>>;
  overwrite_schedule: Option<Vec<Period>>;
  admins: Vec<String>; // user ids
}

struct Period {
  start: NaiveDate;
  end: NaiveDate;
  name: String;
}
```

### Telegram module
```rust
enum Action {
  SetAssignment((String, String)),
  DeleteAssignment(String),
  DeleteSubject(String),

  UpdateTomorrowSchedule(Vec<Period>),
  SetSchedule(Vec<Period>),

  PromoteUserId(String),
  DemoteUserId(String),
}

/// Module Telegram
async fn init(token: &str) -> Bot;

async fn get_updates(bot: Bot, conversations: Vec<ConversationState>) -> Option<Action>;
```