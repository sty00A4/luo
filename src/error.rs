use std::fmt::Display;
use crate::scanning::tokens::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Error(String),
    InputFile(String),
    UnexpectedEOF,
    IllegalChar(char), ExpectedChar(char), ExpectedHexDigit,
    UnexpectedToken(TokenType), ExpectedToken(TokenType, Option<TokenType>)
}
impl Error {
    pub fn error(msg: &str) -> Self { Self::Error(msg.to_string()) }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error(msg) => write!(f, "ERROR: {msg}"),
            Self::InputFile(path) => write!(f, "ERROR: couldn't find input path {path:?}"),
            Self::UnexpectedEOF => write!(f, "ERROR: unexpected end of file"),
            Self::IllegalChar(c) => write!(f, "ERROR: illegal character {c:?}"),
            Self::ExpectedChar(c) => write!(f, "ERROR: expected character {c:?}"),
            Self::ExpectedHexDigit => write!(f, "ERROR: expected hexadecimal digit"),
            Self::UnexpectedToken(token) => write!(f, "ERROR: unexpected {}", token.name()),
            Self::ExpectedToken(expected, got) => write!(f, "ERROR: expected {}{}", expected.name(),
            if let Some(got) = got { format!(", got {}", got.name()) } else { String::new() }),
        }
    }
}