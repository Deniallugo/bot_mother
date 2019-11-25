use std::str::FromStr;

pub enum BotCommand {
    Start,
    Test,
}

impl FromStr for BotCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<BotCommand, ()> {
        match s {
            "/start" => Ok(BotCommand::Start),
            "/test" => Ok(BotCommand::Test),
            _ => Err(()),
        }
    }
}
