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
- [ ] implement schedules [[@pc]]
- [ ] implement assignments [[@pc]]
- [ ] implement admins [[@pc]]
- [ ] implement logging [[@pc]]
- [ ] wrap builds in a flake [[@pc]]
- [ ] initial launch [[@pc]]
- [ ] implement the DB [[@pc]]

## Server owner's manual (self-hosting)
You'll need any machine that has the Nix package manager on it. Personally, I'm using a Raspberry Pi 3B+.

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
For now, it won't have a database to simplify its design. Later on, I plan on adding that.

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
