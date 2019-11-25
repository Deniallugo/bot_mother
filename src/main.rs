use std::collections::HashMap;
use std::env;

use bot_mother::bot::Bot;

use crate::state::State;

mod commands;
mod messages;
mod state;

fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let state = State::new(HashMap::new());
    let mut bot = Bot::new(token, Box::new(state));
    bot.run();
}
