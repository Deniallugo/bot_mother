use std::any::Any;
use std::rc::Rc;

use futures::stream::Stream;
use telegram_bot::*;
use tokio_core::reactor::Core;

use crate::dispatcher::dispatch;
use crate::messages::BotMessage;
use crate::user::User;

pub trait BotState {
    fn get_user(&self, user_id: &UserId) -> Option<Rc<User>>;
    fn add_user(&mut self, user: User) -> Result<(), Box<dyn std::error::Error>>;
    fn set_current_user_message(&self, user: &User, message: Rc<BotMessage>);
    fn get_current_user_message(&self, user: &User) -> Option<Rc<BotMessage>>;
    fn handle_command(&mut self, api: &Api, user: &User, command: &String) -> Option<BotMessage>;
    fn as_any(&mut self) -> &mut dyn Any;
}

pub struct Bot {
    token: String,
    pub state: Box<dyn BotState>,
}

impl Bot {
    pub fn new(token: String, state: Box<dyn BotState>) -> Bot {
        Bot { token, state }
    }
    pub fn run(&mut self) {
        let mut core = Core::new().unwrap();
        println!("Bot is started {}", &self.token);
        let api = Api::configure(&self.token).build(core.handle()).unwrap();
        let future = api.stream().for_each(|update| {
            if dispatch(&api, self, update).is_err() {
                panic!("Something went wrong")
            }
            Ok(())
        });
        core.run(future).unwrap();
    }
}
