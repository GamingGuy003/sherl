pub struct Lexer {}
pub struct Token {
    pub r#type: TokenType,
    pub value: String,
}

pub enum TokenType {
    /// $ *
    Prefix,
    /// "test"
    String,
    /// a = "test"
    Expression,
    /// 0-9
    Number,
}
