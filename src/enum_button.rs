use telegram_bot::KeyboardButton;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "failed to parse '{}' to {}", input, to)]
    ParseStrError { input: String, to: String },
}

pub trait AsButton {
    fn as_button(&self) -> KeyboardButton;
}

#[macro_export]
macro_rules! enum_button {
    ($name:ident, $(($key:ident, $value:expr),)*) => {
       #[derive(Debug, PartialEq)]
       pub enum $name
        {
            $($key),*
        }

        impl AsButton for $name {
            fn as_button(&self) -> telegram_bot::KeyboardButton {
                match self {
                    $(
                        &$name::$key => telegram_bot::KeyboardButton::new($value)
                    ),*
                }
            }
        }

        impl FromStr for $name {
            type Err = Error;

            fn from_str(val: &str) -> Result<Self, Self::Err> {
                match val
                 {
                    $(
                        $value => Ok($name::$key)
                    ),*,
                    _ => Err(Error::ParseStrError{input: val.to_string(), to: stringify!($name).to_string()})
                }
            }
        }
    }
}
