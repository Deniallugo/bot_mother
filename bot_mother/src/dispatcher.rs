use std::rc::Rc;

use telegram_bot::{
    Api, CanReplySendMessage, Message, MessageEntityKind, MessageKind, Update, UpdateKind,
};

use crate::bot::{Bot, BotState};
use crate::user::User;

fn default_handler(api: &Api, message: &Message) {
    if let MessageKind::Text { ref data, .. } = message.kind {
        api.spawn(message.text_reply(format!(
            "Hi, {}! You just wrote '{}'",
            message.from.first_name, data
        )));
    }
}

fn cut_command(data: &String, length: i64, offset: i64) -> String {
    data[offset as usize..(offset + length) as usize].to_string()
}

fn get_or_create_user(
    state: &mut dyn BotState,
    user: &telegram_bot::User,
) -> Result<Rc<User>, Box<dyn std::error::Error>> {
    let user = match state.get_user(&user.id) {
        Some(user) => user,
        None => {
            let bot_user = User::from_telegram_user(user);
            state.add_user(bot_user)?;
            state.get_user(&user.id).unwrap()
        }
    };
    Ok(user)
}

trait ToUser {
    fn to_user(&self, state: &mut dyn BotState) -> Result<Rc<User>, Box<dyn std::error::Error>>;
}

impl ToUser for Update {
    fn to_user(&self, state: &mut dyn BotState) -> Result<Rc<User>, Box<dyn std::error::Error>> {
        match &self.kind {
            UpdateKind::Message(ref message) => get_or_create_user(state, &message.from),
            UpdateKind::CallbackQuery(ref query) => get_or_create_user(state, &query.from),
            UpdateKind::EditedMessage(ref message) => get_or_create_user(state, &message.from),
            UpdateKind::InlineQuery(ref query) => get_or_create_user(state, &query.from),
            _ => panic!("Unsupported types"),
        }
    }
}

//TODO Think about return value
pub fn dispatch(
    api: &Api,
    bot: &mut Bot,
    update: Update,
) -> Result<(), Box<dyn std::error::Error>> {
    let user = update.to_user(bot.state.as_mut())?;

    if let Some(message) = &bot.state.get_current_user_message(&user) {
        let current_message = message.handle(api, bot.state.as_mut(), &user, &update);
        if let Some(current_message) = current_message {
            bot.state
                .set_current_user_message(&user, current_message.clone());
            return Ok(());
        }
    }

    if let UpdateKind::Message(message) = &update.kind {
        if let MessageKind::Text {
            ref data,
            ref entities,
        } = message.kind
        {
            for entity in entities {
                match entity.kind {
                    MessageEntityKind::BotCommand => {
                        if let Some(message) = bot.state.handle_command(
                            api,
                            &user,
                            &cut_command(data, entity.length, entity.offset),
                        ) {
                            bot.state.set_current_user_message(&user, Rc::new(message));
                        }
                        return Ok(());
                    }
                    _ => (),
                }
            }
            // TODO Remove it after tests
            default_handler(api, &message);
        }
    }
    Ok(())
}
