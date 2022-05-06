#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Hash, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
    Init,
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
