use color_eyre::eyre::Result;
use frankenstein::{AllowedUpdate, Api, GetUpdatesParams, GetUpdatesParamsBuilder, TelegramApi};

use crate::Messenger;

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
    fn fetch(&self) -> Result<()> {
        todo!();
    }
    fn last(&self) -> Result<Option<String>> {
        todo!();
    }
    fn reply(&self, message: String) -> Result<()> {
        todo!();
    }
}
