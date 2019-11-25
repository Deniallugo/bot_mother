use bot_mother::bot::BotState;
use bot_mother::messages::{BotMessage, Button};
use bot_mother::user::User;
use telegram_bot::{Api, Update, UserId};

fn first_handler(
    api: &Api,
    user: &User,
    state: &mut dyn BotState,
    update: &Update,
) -> Option<BotMessage> {
    let first_button = Button::new("First Handler".to_owned(), None, false, None);

    let message = BotMessage::new("Message2".to_owned(), vec![first_button], None);
    api.spawn(message.to_send_message(user.chat_id));
    Some(message)
}

fn second_handler(
    api: &Api,
    user: &User,
    state: &mut dyn BotState,
    update: &Update,
) -> Option<BotMessage> {
    let message = BotMessage::new("Message3".to_owned(), vec![], None);
    api.spawn(message.to_send_message(user.chat_id));
    Some(message)
}

pub(crate) fn start_message(api: &Api, chat: UserId) -> BotMessage {
    let first_button = Button::new("First Handler".to_owned(), None, false, Some(first_handler));
    let second_button = Button::new(
        "Second Handler".to_owned(),
        None,
        false,
        Some(second_handler),
    );
    let message = BotMessage::new(
        "Message".to_owned(),
        vec![first_button, second_button],
        None,
    );
    api.spawn(message.to_send_message(chat));
    message
}
