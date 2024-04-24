use super::error::{ParserError, ParserErrorType};

/// The main struct
pub struct Stream {
    pub position: Position,
    pub stream: Vec<char>,
}

/// The current position
#[derive(Default, Clone)]
pub struct Position {
    // the index in the input stream currently being handled
    pub position: usize,
    pub line: usize,
    pub column: usize,
}

impl Stream {
    /// creates a new instance
    pub fn new(stream: Vec<char>) -> Self {
        Self {
            position: Position::default(),
            stream,
        }
    }

    /// peeks the element at current position + offset
    /// (+ 1 will peek the next element)
    pub fn peek(&self, offset: usize) -> Result<char, ParserError> {
        self.stream.get(self.position.position + offset).map_or(
            Err(self.throw_error(
                super::error::ParserErrorType::StreamError,
                "Could not peek. End of stream".to_owned(),
            )),
            |character| Ok(character.clone()),
        )
    }

    /// yields the current element from the stream
    /// will advance position by one
    pub fn next(&mut self) -> Result<char, ParserError> {
        self.stream.get(self.position.position).map_or(
            Err(self.throw_error(
                super::error::ParserErrorType::StreamError,
                "Could not peek. End of stream".to_owned(),
            )),
            |character| {
                if *character == '\n' {
                    // if newline has been reached, increment current line and reset column
                    self.position.line += 1;
                    self.position.column = 0;
                } else {
                    // if newline has not been reached, increment column
                    self.position.column += 1;
                }
                Ok(character.clone())
            },
        )
    }

    /// throws an error and attaches current position
    pub fn throw_error(&self, r#type: ParserErrorType, message: String) -> ParserError {
        ParserError {
            r#type,
            message,
            position: self.position.clone(),
        }
    }
}
