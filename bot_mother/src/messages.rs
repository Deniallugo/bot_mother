use std::fmt;
use std::rc::Rc;
use std::sync::Arc;

use telegram_bot;
use telegram_bot::{
    Api, KeyboardButton, Message, MessageKind, ReplyKeyboardMarkup, ReplyKeyboardRemove,
    SendMessage, ToChatRef, Update, UpdateKind, UserId,
};

use crate::bot::BotState;
use crate::user::User;

type ButtonHandler = fn(&Api, &User, &mut dyn BotState, update: &Update) -> Option<BotMessage>;

pub struct Button {
    text: String,
    next_message: Option<Rc<BotMessage>>,
    inline: bool,
    handler: Option<ButtonHandler>,
}

impl Button {
    pub fn new(
        text: String,
        next_message: Option<Rc<BotMessage>>,
        inline: bool,
        handler: Option<ButtonHandler>,
    ) -> Self {
        Self {
            text,
            next_message,
            inline,
            handler,
        }
    }
}

impl fmt::Debug for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, {:?}", self.text, self.next_message)
    }
}

impl Button {
    pub fn as_button(&self) -> KeyboardButton {
        KeyboardButton::new(self.text.clone())
    }
}

pub struct BotMessage {
    pub(crate) text: String,
    pub(crate) buttons: Vec<Button>,
    default_handler: Option<ButtonHandler>,
}

impl fmt::Debug for BotMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, {:?}", self.text, self.buttons)
    }
}

impl BotMessage {
    pub fn new(text: String, buttons: Vec<Button>, default_handler: Option<ButtonHandler>) -> Self {
        Self {
            text,
            buttons,
            default_handler,
        }
    }

    pub fn to_send_message<C: ToChatRef>(&self, chat: C) -> SendMessage {
        let mut message = SendMessage::new(chat, &self.text);
        if self.buttons.is_empty() {
            let reply_markup = ReplyKeyboardRemove::new();
            message.reply_markup(reply_markup);
        } else {
            let mut reply_markup = ReplyKeyboardMarkup::new();
            for button in &self.buttons {
                reply_markup.add_row(vec![button.as_button()]);
            }
            message.reply_markup(reply_markup);
        }
        message
    }
    pub fn handle(
        &self,
        api: &Api,
        state: &mut dyn BotState,
        user: &User,
        update: &Update,
    ) -> Option<Rc<BotMessage>> {
        let mut result_message;
        if let UpdateKind::Message(message) = &update.kind {
            if let MessageKind::Text { data, .. } = &message.kind {
                for button in &self.buttons {
                    if button.text == *data {
                        if let Some(handler) = button.handler {
                            result_message = match handler(api, user, state, update) {
                                Some(message) => Some(Rc::new(message)),
                                None => None,
                            }
                        }
                    }
                }
            }
        }
        if let Some(handler) = self.default_handler {
            result_message = match handler(api, &user, state, update) {
                Some(message) => Some(Rc::new(message)),
                None => None,
            }
        } else {
            result_message = None;
        }
        return result_message;
    }
}
