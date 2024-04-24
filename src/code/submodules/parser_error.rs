use std::fmt::Display;

pub struct ParserError {
    pub error_type: ParserErrorType,
    pub position: ParserErrorPosition,
    pub message: String,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.error_type.clone() {
            ParserErrorType::Other(message) => {
                write!(f, "{:?} at {}: {}", message, self.position, self.message)
            }
            _ => write!(
                f,
                "{:?} at {}: {}",
                self.error_type, self.position, self.message
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParserErrorType {
    InputStream,
    Lexer,
    Parser,
    Other(String),
}

pub struct ParserErrorPosition {
    pub line: usize,
    pub column: usize,
    pub character: Option<char>,
}

impl Display for ParserErrorPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.character {
            Some(character) => {
                write!(f, "{}:{}, Character {}", self.line, self.column, character)
            }
            None => write!(f, "{}:{}", self.line, self.column),
        }
    }
}
