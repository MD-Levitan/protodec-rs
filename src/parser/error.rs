use core::fmt::{self, Debug, Display};

pub type Result<T> = core::result::Result<T, Error>;

pub enum ErrorType {
    GeneralError,
    ParserError,
    GeneratorError,
    IncorrectType,
    IncorrectData,
}

#[derive(Default)]
pub struct Error {
    details: String,
    type_: ErrorType,
}

impl Default for ErrorType {
    fn default() -> ErrorType {
        ErrorType::GeneralError
    }
}

impl Error {
    /// Create a new error with no associated source
    pub fn new(msg: &str, type_: Option<ErrorType>) -> Self {
        Error {
            details: msg.to_string(),
            type_: type_.unwrap_or(ErrorType::GeneralError),
        }
    }
}

impl Debug for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                ErrorType::GeneralError => "GeneralError",
                ErrorType::ParserError => "ParserError",
                ErrorType::GeneratorError => "GeneratorError",
                ErrorType::IncorrectType => "IncorrectType",
                ErrorType::IncorrectData => "IncorrectData",
            }
        )
    }
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                ErrorType::GeneralError => "GeneralError",
                ErrorType::ParserError => "ParserError",
                ErrorType::GeneratorError => "GeneratorError",
                ErrorType::IncorrectType => "IncorrectType",
                ErrorType::IncorrectData => "IncorrectData",
            }
        )
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error::{} - {}", self.type_, self.details)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error: type - {}, message - {}",
            self.type_, self.details
        )
    }
}
