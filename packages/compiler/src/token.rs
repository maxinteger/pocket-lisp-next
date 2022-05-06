use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Hash, Copy, Clone)]
pub enum TokenType {
    Init,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftSquare,
    RightSquare,
    Dispatch,
    True,
    False,
    Identifier,
    Keyword,
    String,
    IntegerNumber,
    FloatNumber,
    FractionNumber,
    Error,
    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Init => f.write_str("Init"),
            TokenType::LeftParen => f.write_str("LeftParen"),
            TokenType::RightParen => f.write_str("RightParen"),
            TokenType::LeftBrace => f.write_str("LeftBrace"),
            TokenType::RightBrace => f.write_str("RightBrace"),
            TokenType::LeftSquare => f.write_str("LeftSquare"),
            TokenType::RightSquare => f.write_str("RightSquare"),
            TokenType::Dispatch => f.write_str("Dispatch"),
            TokenType::True => f.write_str("True"),
            TokenType::False => f.write_str("False"),
            TokenType::Identifier => f.write_str("Identifier"),
            TokenType::Keyword => f.write_str("Keyword"),
            TokenType::String => f.write_str("String"),
            TokenType::IntegerNumber => f.write_str("IntegerNumber"),
            TokenType::FloatNumber => f.write_str("FloatNumber"),
            TokenType::FractionNumber => f.write_str("FractionNumber"),
            TokenType::Error => f.write_str("Error"),
            TokenType::Eof => f.write_str("Eof"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Token<'a> {
    pub kind: TokenType,
    pub start: usize,
    pub src: &'a str,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenType, start: usize, src: &'a str, line: usize) -> Self {
        Token {
            kind,
            start,
            src,
            line,
        }
    }
}

impl Default for Token<'static> {
    fn default() -> Token<'static> {
        Token {
            kind: TokenType::Init,
            start: 0,
            src: "",
            line: 0,
        }
    }
}
