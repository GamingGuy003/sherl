use super::parser_error::{ParserError, ParserErrorPosition, ParserErrorType};

/// the struct to store data
pub struct InputStream {
    /// the current position
    pub position: usize,
    /// the current line
    pub line: usize,
    /// the current column
    pub column: usize,
    /// input stream
    pub input: Vec<char>,
}

impl InputStream {
    /// creates a new instance
    pub fn new(input: String) -> Self {
        Self {
            position: 0,
            line: 1,
            column: 0,
            input: input.chars().collect(),
        }
    }

    /// looks ahead without consuming
    pub fn peek(&self) -> Result<char, ParserError> {
        // returns next character or throws error
        self.input.get(self.position).map_or(
            Err(ParserError {
                error_type: ParserErrorType::InputStream,
                position: ParserErrorPosition {
                    line: self.line,
                    column: self.column + 1,
                    character: None,
                },
                message: "Could not peek next character".to_owned(),
            }),
            |character| Ok(character.clone()),
        )
    }

    /// gets token while also consuming
    pub fn next(&mut self) -> Result<char, ParserError> {
        let next_character = self.input.get(self.position + 1).map_or(
            Err(ParserError {
                error_type: ParserErrorType::InputStream,
                position: ParserErrorPosition {
                    line: self.line,
                    column: self.column + 1,
                    character: None,
                },
                message: "Could not peek next character".to_owned(),
            }),
            |character| Ok(character.clone()),
        )?;

        if next_character == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }

        Ok(next_character)
    }

    /// checks if EOF has been reached
    pub fn is_eof(&self) -> bool {
        // we assume we reached eof when peek errors
        self.peek().is_err()
    }

    pub fn throw_error(&self, error_type: ParserErrorType, message: String) -> ParserError {
        ParserError {
            error_type,
            position: ParserErrorPosition {
                line: self.line,
                column: self.column,
                character: None,
            },
            message,
        }
    }
}
