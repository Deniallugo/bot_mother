use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;

use bot_mother::bot::BotState;
use bot_mother::messages::BotMessage;
use bot_mother::user::User;
use telegram_bot::{Api, UserId};

use crate::commands::BotCommand;
use crate::messages::start_message;

#[derive(Debug, Default)]
pub struct State {
    pub users: HashMap<UserId, Rc<User>>,
    pub user_message: RefCell<HashMap<UserId, Rc<BotMessage>>>,
}

impl BotState for State {
    fn get_user(&self, user_id: &UserId) -> Option<Rc<User>> {
        if let Some(user) = self.users.get(&user_id) {
            return Some(user.clone());
        } else {
            None
        }
    }
    fn add_user(&mut self, user: User) -> Result<(), Box<dyn std::error::Error>> {
        self.users.insert(user.chat_id, Rc::new(user));
        Ok(())
    }

    fn set_current_user_message(&self, user: &User, message: Rc<BotMessage>) {
        self.user_message.borrow_mut().insert(user.chat_id, message);
    }

    fn get_current_user_message(&self, user: &User) -> Option<Rc<BotMessage>> {
        if let Some(message) = self.user_message.borrow().get(&user.chat_id) {
            return Some(message.clone());
        } else {
            None
        }
    }

    fn handle_command(&mut self, api: &Api, user: &User, command: &String) -> Option<BotMessage> {
        match BotCommand::from_str(command) {
            Ok(BotCommand::Start) => Some(start_message(api, user.chat_id)),
            _ => None,
        }
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl State {
    pub fn new(users: HashMap<UserId, Rc<User>>) -> State {
        State {
            users,
            user_message: RefCell::new(HashMap::default()),
        }
    }
}
