use telegram_bot;
use telegram_bot::{
    Api, KeyboardButton, Message, MessageKind, ReplyKeyboardMarkup, ReplyKeyboardRemove,
    SendMessage, ToChatRef, Update, UpdateKind, UserId,
};

use crate::bot::BotState;
use crate::messages::BotMessage;

#[derive(Debug, Clone)]
pub struct User {
    username: Option<String>,
    pub first_name: String,
    last_name: Option<String>,
    pub chat_id: UserId,
}

impl User {
    pub fn from_message(message: &Message) -> Self {
        Self::from_telegram_user(&message.from)
    }

    pub fn from_telegram_user(user: &telegram_bot::User) -> Self {
        Self {
            chat_id: user.id,
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            username: user.username.clone(),
        }
    }
}
