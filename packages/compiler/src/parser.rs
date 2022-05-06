use std::collections::HashMap;
use std::str::FromStr;
use crate::{Chunk, Code, OpCode, Scanner, scanner, Token, TokenType, Value};

pub struct Parser<'a> {
    scanner: &'a mut Scanner<'a>,
    previous: Token<'a>,
    current: Token<'a>,
    had_error: bool,
    panic_mode: bool,
    chunk: &'a mut Chunk,
    rules: HashMap<TokenType, ParseRule>,
    last_error: String
}

impl<'a> Parser<'a> {
    pub fn new(scanner: &'a mut scanner::Scanner<'a>, chunk: &'a mut Chunk) -> Self {
        Parser {
            previous: Token::default(),
            current: Token::default(),
            scanner,
            had_error: false,
            panic_mode: false,
            chunk,
            rules: Parser::get_rules(),
            last_error: "".to_owned()
        }
    }

    pub fn parse(&mut self) -> bool {
        self.advance();
        self.expression();
        self.consume(TokenType::Eof, "Expect end of expression.");
        self.end();
        !self.had_error
    }

    fn get_rules() -> HashMap<TokenType, ParseRule> {
        let mut map = HashMap::new();
        use TokenType::*;

        map.insert(
            LeftParen,
            ParseRule::new(ParseFn::Groping, ParseFn::None, Precedence::None),
        );
        map.insert(
            RightParen,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            LeftBrace,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            RightBrace,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Comma,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Dot,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Minus,
            ParseRule::new(ParseFn::Unary, ParseFn::Binary, Precedence::Term),
        );
        map.insert(
            Plus,
            ParseRule::new(ParseFn::None, ParseFn::Binary, Precedence::Term),
        );
        map.insert(
            Semicolon,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Slash,
            ParseRule::new(ParseFn::None, ParseFn::Binary, Precedence::Factor),
        );
        map.insert(
            Star,
            ParseRule::new(ParseFn::None, ParseFn::Binary, Precedence::Factor),
        );
        map.insert(
            Bang,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            BangEqual,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Equal,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            EqualEqual,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Greater,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            GreaterEqual,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Less,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            LessEqual,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Identifier,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            String,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Number,
            ParseRule::new(ParseFn::Number, ParseFn::None, Precedence::None),
        );
        map.insert(
            And,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Class,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Else,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            False,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            For,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Fun,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            If,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Nil,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Or,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Print,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Return,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Super,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            This,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            True,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Var,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            While,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Error,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map.insert(
            Eof,
            ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None),
        );
        map
    }

    fn emit_byte(&mut self, byte: Code) {
        self.chunk.push_chunk(byte, self.previous.line);
    }

    fn emit_bytes(&mut self, byte1: Code, byte2: Code) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn end(&mut self) {
        self.emit_byte(OpCode::Return as Code);

        if cfg!(feature = "debug_print_code") && !self.had_error {
            self.chunk.disassemble("code");
        }
    }

    fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = self.scanner.scan_token();
            if self.current.kind != TokenType::Error {
                break;
            }

            self.error_at_current(self.current.src)
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(&Precedence::Assignment)
    }

    fn consume(&mut self, kind: TokenType, message: &str) {
        if self.current.kind == kind {
            self.advance()
        } else {
            self.error_at_current(message)
        }
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.previous, message)
    }

    fn error_at(&mut self, token: Token, message: &str) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;
        let line_prefix = format!("[line {}] Error", token.line);

        let token = if token.kind == TokenType::Eof {
            " at end".to_owned()
        } else if token.kind == TokenType::Error {
            todo!()
        } else {
            format!(" at '{}'", token.src)
        };

        self.had_error = true;
        self.last_error = format!("{}{}: {}", line_prefix, token, message);

        eprintln!("{}", self.last_error);
    }

    fn number(&mut self) {
        let value: Value = Value::from_str(self.previous.src).unwrap();
        self.emit_constant(value)
    }

    fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::Constant as Code, constant)
    }
    fn make_constant(&mut self, value: Value) -> Code {
        let constant_idx = self.chunk.push_constant(value);
        if constant_idx > u8::MAX.into() {
            self.error_at_current("Too many constants in one chunk")
        }
        constant_idx.try_into().unwrap()
    }

    fn parse_precedence(&mut self, precedence: &Precedence) {
        self.advance();
        let token = self.previous.kind;
        let prefix_rule = self.get_rule(&token);

        self.execute_parse_fn(&prefix_rule.prefix, true);

        while precedence <= &self.get_rule(&self.current.kind).precedence {
            self.advance();
            let infix_rule = &self.get_rule(&self.previous.kind).infix;
            self.execute_parse_fn(infix_rule, false)
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.")
    }

    fn unary(&mut self) {
        let op_kind = self.previous.kind;

        self.expression();

        if op_kind == TokenType::Minus { self.emit_byte(OpCode::Negate as Code) }
    }

    fn binary(&mut self) {
        let op_kind = self.previous.kind;
        let rule = self.get_rule(&op_kind);
        self.parse_precedence(&rule.get_next_precedence());

        match op_kind {
            TokenType::Plus => self.emit_byte(OpCode::Add as Code),
            TokenType::Minus => self.emit_byte(OpCode::Subtract as Code),
            TokenType::Star => self.emit_byte(OpCode::Multiply as Code),
            TokenType::Slash => self.emit_byte(OpCode::Divide as Code),
            _ => {},
        }
    }

    fn get_rule(&self, token_type: &TokenType) -> ParseRule {
        *self.rules.get(token_type).expect("Missing parser rule")
    }

    fn execute_parse_fn(&mut self, fun: &ParseFn, strict: bool) {
        match fun {
            ParseFn::None => {
                if strict {
                    self.error_at_current("Expect expression.")
                }
            }
            ParseFn::Groping => self.grouping(),
            ParseFn::Unary => self.unary(),
            ParseFn::Binary => self.binary(),
            ParseFn::Number => self.number(),
        }
    }
}



#[repr(u8)]
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
#[allow(dead_code)]
enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
enum ParseFn {
    None,
    Groping,
    Unary,
    Binary,
    Number,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
struct ParseRule {
    prefix: ParseFn,
    infix: ParseFn,
    precedence: Precedence,
}

impl ParseRule {
    fn new(prefix: ParseFn, infix: ParseFn, precedence: Precedence) -> Self {
        ParseRule {
            prefix,
            infix,
            precedence,
        }
    }

    fn get_next_precedence(&self) -> Precedence {
        let prec = &self.precedence;
        let next = if prec == &Precedence::Primary {
            (*prec) as u8
        } else {
            ((*prec) as u8) + 1
        };
        let precedence: Precedence = unsafe { ::std::mem::transmute(next) };
        precedence
    }
}



#[cfg(test)]
mod parser_tests {
    use crate::{Scanner, Parser, Chunk, OpCode};

    fn parse(source: &str) -> (bool, String, Chunk) {
        let mut chunks = Chunk::new();
        let mut scanner = Scanner::new(source);
        let mut parser = Parser::new(&mut scanner, &mut chunks);
        let parse_result = parser.parse();
        (parse_result, parser.last_error, chunks)
    }

    #[test]
    fn parse_empty_source() {
        let (result, last_error, _) = parse("");

        assert_eq!(result, false);
        assert_eq!(last_error, "[line 1] Error at end: Expect expression.")
    }

    #[test]
    fn parse_one_constant () {
        let (result, _, chunks) = parse("42");

        let mut expected_chunks = Chunk::new();
        expected_chunks.push_op_code(OpCode::Constant, 1);
        expected_chunks.push_chunk(0, 1); // index of the constant
        expected_chunks.push_constant(42.0);
        expected_chunks.push_op_code(OpCode::Return, 1);

        assert_eq!(result, true);
        assert_eq!(chunks, expected_chunks)
    }

    #[test]
    fn parse_expression () {
        let (result, _, chunks) = parse("((-1) + 2) * 3 - -4" );

        let mut expected_chunks = Chunk::new();
        expected_chunks.push_op_code(OpCode::Constant, 1);
        expected_chunks.push_chunk(0, 1); // index of the constant
        expected_chunks.push_constant(1.0);

        expected_chunks.push_op_code(OpCode::Negate, 1);

        expected_chunks.push_op_code(OpCode::Constant, 1);
        expected_chunks.push_chunk(1, 1); // index of the constant
        expected_chunks.push_constant(2.0);

        expected_chunks.push_op_code(OpCode::Add, 1);

        expected_chunks.push_op_code(OpCode::Constant, 1);
        expected_chunks.push_chunk(2, 1); // index of the constant
        expected_chunks.push_constant(3.0);

        expected_chunks.push_op_code(OpCode::Multiply, 1);

        expected_chunks.push_op_code(OpCode::Constant, 1);
        expected_chunks.push_chunk(3, 1); // index of the constant
        expected_chunks.push_constant(4.0);

        expected_chunks.push_op_code(OpCode::Negate, 1);
        expected_chunks.push_op_code(OpCode::Subtract, 1);
        expected_chunks.push_op_code(OpCode::Return, 1);

        assert_eq!(result, true);
        assert_eq!(chunks, expected_chunks)
    }
}


#[cfg(test)]
mod parse_rule_tests {
    use crate::parser::{ParseFn, ParseRule, Precedence};

    #[test]
    fn test_get_next_precedence() {
        let rule = ParseRule::new(ParseFn::None, ParseFn::None, Precedence::None);
        assert_eq!(rule.get_next_precedence(), Precedence::Assignment);

        let rule = ParseRule::new(ParseFn::None, ParseFn::None, Precedence::Call);
        assert_eq!(rule.get_next_precedence(), Precedence::Primary);

        let rule = ParseRule::new(ParseFn::None, ParseFn::None, Precedence::Primary);
        assert_eq!(rule.get_next_precedence(), Precedence::Primary);
    }

}