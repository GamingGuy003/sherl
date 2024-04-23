use super::{input_stream::InputStream, parser_error::ParserError};

pub struct TokenStream {
    pub input_stream: InputStream,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub enum TokenType {
    Punctuation,
    Number,
    String,
    Keyword,
    Variable,
    Operation,
}

impl TokenStream {
    // reads as long as predicate is true
    pub fn read_while<F>(&mut self, predicate: F) -> Result<String, ParserError>
    where
        F: Fn(char) -> bool,
    {
        let mut string = String::new();
        while !self.input_stream.is_eof() && predicate(self.input_stream.peek()?) {
            string.push(self.input_stream.next()?);
        }
        Ok(string)
    }

    // determines what to do with the next token
    pub fn read_next(&mut self) -> Result<Option<Token>, ParserError> {
        self.read_while(|char| char.is_whitespace())?;
        if self.input_stream.is_eof() {
            return Ok(None);
        }
        let next_char = self.input_stream.peek()?;
        match next_char {
            _ if next_char == '#' => {
                self.skip_comment()?;
                Ok(self.read_next()?)
            }
            _ if next_char == '"' => Ok(Some(self.read_string()?)),
            _ if next_char.is_digit(10) => Ok(Some(self.read_number()?)),
            _ => Ok(None),
        }
    }

    // skips until newline is found
    pub fn skip_comment(&mut self) -> Result<(), ParserError> {
        self.read_while(|char| char != '\n')?;
        self.input_stream.next()?;
        Ok(())
    }

    // reads until escape character is found
    pub fn read_escaped(&mut self, end: char) -> Result<String, ParserError> {
        let mut string = String::new();
        let mut escaped = false;
        self.input_stream.next()?;
        while !self.input_stream.is_eof() {
            let next_character = self.input_stream.next()?;
            if escaped {
                string.push(next_character);
                escaped = false;
            } else if next_character == '\\' {
                escaped = true;
            } else if next_character == end {
                break;
            } else {
                string.push(next_character);
            }
        }
        Ok(string)
    }

    // reads a string
    pub fn read_string(&mut self) -> Result<Token, ParserError> {
        Ok(Token {
            token_type: TokenType::String,
            value: self.read_escaped('"')?,
        })
    }

    // reads a number
    pub fn read_number(&mut self) -> Result<Token, ParserError> {
        let mut has_dot = false;
        let number = self.read_while(|char| {
            if char == '.' {
                if has_dot {
                    return false;
                }
                has_dot = true;
                return true;
            }
            char.is_digit(10)
        })?;
    }
}