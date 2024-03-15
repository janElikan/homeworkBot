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
- [x] create conversation state [[@pc]]
- [x] define the telegram module [[@phone]]
- [x] move telegram things into a separate module
- [x] figure out how to actually fix the problem described below
- [x] redo the types
- [ ] write the state module, make it work in-ram [[@pc]]
- [ ] write the telegram module [[@pc]]
- [ ] write the conversation logic [[@pc]]
- [ ] add logging [[@pc]]
- [ ] improve error types [[@pc]]
- [ ] implement persistent storage [[@pc]]
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

![diagram](assets/diagram.excalidraw.png)

Modules:
- state
- telegram
