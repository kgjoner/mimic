use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use super::print_message;

#[derive(Debug)]
pub enum NormalizedError {
    EmptyAction(Option<String>),
    InvalidFlag(Option<String>),
    RequiredActionArg(Option<String>),
    RequiredFlagArg(Option<String>),
    Unexpected(Option<String>),
    UnknownAction(Option<String>),
}

impl NormalizedError {
    pub fn new(err: Box<dyn Error>) -> NormalizedError {
        *err.downcast::<NormalizedError>()
            .unwrap_or_else(|err| Box::new(NormalizedError::Unexpected(Some(err.to_string()))))
    }

    fn message(&self) -> String {
        let (talk, info) = match &self {
            NormalizedError::EmptyAction(s) => ("Err... What do'ya need?", s),
            NormalizedError::InvalidFlag(s) => ("Ew... What should I do with it?", s),
            NormalizedError::RequiredActionArg(s) | NormalizedError::RequiredFlagArg(s) => {
                ("Hum... Will'ya finish saying your needs?", s)
            }
            NormalizedError::Unexpected(s) => ("Blarg! It was not supposed to happen that!", s),
            NormalizedError::UnknownAction(s) => ("Err... I dunno what'ya say!", s),
        };

        let info = if let Some(s) = info { &s[..] } else { "" };

        format!(
            "\
<talk>{talk}<r>
{info}
"
        )
    }

    pub fn print(&self) {
        print_message(&self.message(), true);
    }
}

impl Error for NormalizedError {}

impl Display for NormalizedError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
