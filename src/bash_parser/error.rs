use std::fmt::Display;

use super::stream::Position;

pub struct ParserError {
    pub r#type: ParserErrorType,
    pub message: String,
    pub position: Position,
}

#[derive(Debug, Clone)]
pub enum ParserErrorType {
    StreamError,
    ParserError,
    LexerError,
    Other(String),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.r#type, self.message)
    }
}
