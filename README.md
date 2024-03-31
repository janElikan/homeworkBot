> A homework assignment keeping telegram bot

Our class in high school has a problem with assignments. Teachers would constantly forget to put them into the system they were supposed to use and tell us to remember the assignments instead. No-one did, so they kept asking me. I grew tired of that, so I made this:

\<a video clip showing how it works />

Features:
- setting/getting assignments
- changing the schedule for tomorrow (happened a lot of times and was not supported by the system the school used)
- notifications when an admin added an assignment

## Progress
- [x] define the requirements
- [x] set up the project (flake)
- [x] get/set
- [x] refactor
- [x] setting schedules
- [x] schedule support for `/get`
- [x] alias `/tomorrow` to `/get` for familiarity (the last bot I wrote for the same class had `/tomorrow`)
- [x] logging
- [x] persistent storage
- [ ] `/all` command
- [ ] add date support to `/get`
- [ ] alpha-release
- [ ] switch command args from string to enum
- [ ] split process_message
- [ ] admins
- [ ] timetables
- [ ] subject name guessing (schedule support for `/set`)
- [ ] refactor
- [ ] stable release

note: add /cancel command
and /delete

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

## Conversation examples
```text
user: /set sci
bot:  what's the assignment?
user: section 8
bot:  saved
```

```text
user: /get
bot: due tomorrow:
bot: english: section 16's grammar
bot: history: section 32
bot: science: task 128
bot: (button saying "next day")
```
But if they ask it in the middle of 2nd pair, it should respond with:
```text
user: /get
bot: due today
bot: history: section 32
bot: science: task 128
```

### Rethinking the implementation
Let's start by defining the valid states of our system.
Frankenstein gives us a few primiteves to work with:
```rust
struct Message {
    from: User,
    chat: Chat,
    text: Option<String>,
    document: Option<Document>, // todo on that
    contact: Option<Contact>,
}

struct User {
    id: u64,
    first_name: String,
    last_name: Option<String>,
}

struct Chat {
    id: i64,
}

struct Document {
    file_id: String,
    file_unique_id: String,
    file_name: Option<String>,
    mime_type: Option<String>,
}

struct Contact {
    user_id: Option<u64>,
    first_name: String,
    last_name: Option<String>,
}
```

And we need to have these:
```rust
struct App {
    assignments: HashMap<String, Assignment>,
    users: HashMap<u64, User>,
    chats: HashMap<u64, Vec<String>>,
    schedule: HashMap<Weekday, Day>,
}

struct Assignment {
    text: String,
    attachments: Vec<String>, // of UUIDs
}

struct Day {
    timetable: Vec<NaiveTime, NaiveTime>,
    periods: Vec<Option<String>>,
}

struct User {
    first_name: String,
    last_name: Option<String>,
    role: Role,
}

enum Role {
    Banned,
    User,
    Admin,
}

enum Argument {
    Text(String),
    User(u64),
    Weekday(Weekday),
}
```

The main function stores:
- App instance
- Frankenstein.Api instance

the message is processed by a function passed into `Api.get_updates`, kinda like [this](https://github.com/ayrat555/frankenstein/blob/master/examples/async_reply_to_message_updates.rs). As the project gets more complex, I'll split it. The goal for now is to *get it to work*.