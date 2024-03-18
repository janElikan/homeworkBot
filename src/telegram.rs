use color_eyre::eyre::{eyre, OptionExt, Result};
use frankenstein::{
    AllowedUpdate, Api, GetUpdatesParams, ReplyParameters, SendMessageParams, TelegramApi, Update,
    UpdateContent,
};

use crate::{Messenger, MessengerUpdate};

pub struct Telegram {
    api: Api,
    updates: Vec<Update>,
    last_processed_message: Option<u32>,
}

impl Telegram {
    pub fn new(token: &str) -> Result<Self> {
        let api = Api::new(token);

        api.get_me()?;

        Ok(Self {
            api,
            updates: Vec::new(),
            last_processed_message: None,
        })
    }
}

impl Messenger for Telegram {
    fn fetch(&mut self) -> Result<()> {
        let update_params =
            GetUpdatesParams::builder().allowed_updates(vec![AllowedUpdate::Message]);

        let update_params = match self.last_processed_message {
            Some(last_msg) => update_params.offset(last_msg + 1).build(),
            None => update_params.build(),
        };

        self.updates = self.api.get_updates(&update_params)?.result;

        Ok(())
    }

    fn first(&self) -> Option<MessengerUpdate> {
        let Some(update) = self.updates.first() else {
            return None;
        };
        let UpdateContent::Message(message) = &update.content else {
            return None;
        };
        let user_id = message
            .from
            .clone()
            .expect("couldn't read the sender id")
            .id
            .to_string();
        let chat_id = message.chat.id.to_string();

        if let Some(contact) = &message.contact {
            let content = contact.user_id?.to_string();
            return Some(MessengerUpdate { chat_id, user_id, content });
        }

        message.text.as_ref().map(|text| MessengerUpdate {
            chat_id,
            user_id,
            content: text.clone(),
        })
    }

    fn reply(&mut self, text: String) -> Result<()> {
        let update = self.updates.first().ok_or_eyre("Nothing to reply to")?;
        let UpdateContent::Message(message) = &update.content else {
            return Err(eyre!("Message is empty"));
        };

        let reply_parameters = ReplyParameters::builder()
            .message_id(message.message_id)
            .build();
        let send_message_params = SendMessageParams::builder()
            .chat_id(message.chat.id)
            .text(text)
            .reply_parameters(reply_parameters)
            .build();

        self.api.send_message(&send_message_params)?;

        self.last_processed_message = Some(update.update_id);

        Ok(())
    }
}
