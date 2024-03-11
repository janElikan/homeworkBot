use color_eyre::eyre::Result;
use frankenstein::{
    AllowedUpdate, Api, GetUpdatesParams, GetUpdatesParamsBuilder, Message, SendMessageParams,
    TelegramApi, UpdateContent,
};

use crate::conversation::{process_message, BotResponse, ChatState};
use crate::{Action, Messenger};

pub struct Telegram {
    api: Api,
    update_params_builder: GetUpdatesParamsBuilder,
    update_params: GetUpdatesParams,
}

impl Telegram {
    pub fn new(token: &str) -> Result<Self> {
        let api = Api::new(token);

        api.get_me()?;

        let update_params_builder = GetUpdatesParams::builder();

        let update_params = update_params_builder
            .clone()
            .allowed_updates(vec![AllowedUpdate::Message])
            .build();

        Ok(Self {
            api,
            update_params_builder,
            update_params,
        })
    }
}

impl Messenger for Telegram {
    fn get_updates(&mut self) -> Result<Option<crate::Action>> {
        let Some(update) = self.api.get_updates(&self.update_params)?.result.pop() else {
            return Ok(None)
        };

        let mut action: Option<Action> = None;

        if let UpdateContent::Message(message) = update.content {
            dbg!(&message);

            let bot_response = match message {
                Message {
                    text: Some(text), ..
                } => process_message(&ChatState::Default, &text),
                _ => BotResponse {
                    reply: "I don't know how to handle this yet",
                    action: None,
                    chat_state: ChatState::Default,
                },
            };

            action = bot_response.action;

            let params = SendMessageParams::builder()
                .chat_id(message.chat.id)
                .text(bot_response.reply)
                .build();

            self.api.send_message(&params)?;
        };

        self.update_params = self
            .update_params_builder
            .clone()
            .offset(update.update_id + 1)
            .build();

        Ok(action)
    }
}
