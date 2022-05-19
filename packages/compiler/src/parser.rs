use crate::scanner::Scanner;
use crate::token::{Token, TokenType};
use anyhow::{Error, Result};
use std::thread::{current, park};

type Program = Vec<ExpressionList>;

type ExpressionList = Vec<ExpressionNode>;

#[derive(Debug, PartialEq)]
enum ExpressionNode {
    Empty,
    BooleanLiteral(bool),
    IntegerNumberLiteral(i64),
    FloatNumberLiteral(f64),
    FractionNumberLiteral(i64, i64),
    StringLiteral(String),
    Identifier(String),
    Keyword(String),
    FunctionCall(ExpressionList),
    AnonymousFunction(ExpressionList),
    Array(ExpressionList),
    Map(ExpressionList),
}

pub struct Parser<'a> {
    scanner: &'a mut Scanner<'a>,
    current: Token<'a>,
    program: Program,
    had_error: bool,
    panic_mode: bool,
    last_error: String,
}

impl<'a> Parser<'a> {
    pub fn new(scanner: &'a mut Scanner<'a>) -> Self {
        Parser {
            current: Token::default(),
            scanner,
            program: vec![],
            had_error: false,
            panic_mode: false,
            last_error: "".to_owned(),
        }
    }

    pub fn parse(&mut self) -> Result<&Program> {
        self.advance();
        while !self.is_end() {
            println!("PARSE {}", self.current.src);
            // top level expression must be lists
            let result = self.expression_list(TokenType::LeftParen);
            if self.had_error {
                break;
            }
            match result {
                Ok(expression) => self.program.push(expression),
                Err(error) => {
                    self.error_at_current(error.to_string().as_str());
                    break;
                }
            }
        }
        self.consume(TokenType::Eof, "Expect end of expression.");
        if self.had_error {
            Err(Error::msg(format!("{}", self.last_error)))
        } else {
            Ok(&self.program)
        }
    }

    fn expression(&mut self) -> Result<ExpressionNode> {
        let token = self.current;
        println!("EXPRESS {}", token.src);
        return match token.kind {
            TokenType::True => {
                self.advance();
                Ok(ExpressionNode::BooleanLiteral(true))
            }
            TokenType::False => {
                self.advance();
                Ok(ExpressionNode::BooleanLiteral(false))
            }
            TokenType::String => {
                let val = token.src.to_owned();
                self.advance();
                Ok(ExpressionNode::StringLiteral(val))
            }
            TokenType::IntegerNumber => {
                let val = token.src.parse::<i64>().expect("Integer number token");
                self.advance();
                Ok(ExpressionNode::IntegerNumberLiteral(val))
            }
            TokenType::FloatNumber => {
                let val = token.src.parse::<f64>().expect("Float number token");
                self.advance();
                Ok(ExpressionNode::FloatNumberLiteral(val))
            }
            TokenType::FractionNumber => {
                let val: Vec<i64> = token
                    .src
                    .split("/")
                    .map(|num| num.parse::<i64>().expect("Integer number token "))
                    .collect();
                self.advance();
                Ok(ExpressionNode::FractionNumberLiteral(val[0], val[1]))
            }
            TokenType::Identifier => {
                let val = token.src.to_owned();
                self.advance();
                Ok(ExpressionNode::Identifier(val))
            }
            TokenType::Keyword => {
                let val = token.src.to_owned();
                self.advance();
                Ok(ExpressionNode::Keyword(val))
            }
            TokenType::Dispatch => {
                println!("DIS 0");
                self.advance();
                println!("DIS 1 {}", self.current.src);
                match self.peek().kind {
                    TokenType::LeftParen => {
                        let exp = self.expression_list(TokenType::LeftParen)?;
                        Ok(ExpressionNode::AnonymousFunction(exp))
                    }
                    _ => self.error_unexpected_token(),
                }
            }
            TokenType::LeftParen => {
                let list = self.expression_list(token.kind)?;
                Ok(ExpressionNode::FunctionCall(list))
            }
            TokenType::LeftSquare => {
                let list = self.expression_list(token.kind)?;
                Ok(ExpressionNode::Array(list))
            }
            TokenType::LeftBrace => {
                let list = self.expression_list(token.kind)?;
                Ok(ExpressionNode::Map(list))
            }
            TokenType::Eof => Ok(ExpressionNode::Empty),
            _ => self.error_unexpected_token(),
        };
    }

    fn advance(&mut self) {
        loop {
            self.current = self.scanner.scan_token();
            if self.current.kind == TokenType::Error {
                self.error_at_current(self.current.src);
            } else {
                break;
            }
        }
    }

    fn expression_list(&mut self, start_token: TokenType) -> Result<ExpressionList> {
        let end_token = match start_token {
            TokenType::LeftParen => TokenType::RightParen,
            TokenType::LeftBrace => TokenType::RightBrace,
            TokenType::LeftSquare => TokenType::RightSquare,
            _ => panic!("Invalid start token for advance until: {}", start_token),
        };
        let mut items = vec![];
        if self.current.kind != start_token {
            self.error_at_current(
                format!("Expected {}, but get {}", start_token, self.current.kind).as_str(),
            );
            return Ok(items);
        }
        self.advance();
        println!("HELLO {}", self.current.src);
        loop {
            if self.current.kind == end_token || self.is_end() {
                break;
            } else {
                println!("EXP");
                let exp = self.expression()?;
                items.push(exp)
            }
        }
        if self.current.kind != end_token {
            self.error_at_current(
                format!("Expected {}, but get {}", end_token, self.current.kind).as_str(),
            )
        } else {
            self.advance();
        }
        Ok(items)
    }

    fn consume(&mut self, kind: TokenType, message: &str) {
        if self.current.kind == kind {
            self.advance()
        } else if !self.had_error {
            self.error_at_current(message)
        }
    }

    fn peek(&self) -> Token {
        self.current
    }

    fn is_end(&self) -> bool {
        self.peek().kind == TokenType::Eof
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.current, message)
    }

    fn error_unexpected_token(&mut self) -> Result<ExpressionNode> {
        Err(Error::msg(format!(
            "Unexpected token {}",
            self.current.kind
        )))
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
            format!("{}", token.src)
        } else {
            format!(" at '{}'", token.src)
        };

        self.had_error = true;
        self.last_error = format!("{}{}: {}", line_prefix, token, message);

        eprintln!("{}", self.last_error);
    }
}

#[cfg(test)]
pub mod tests {
    use crate::parser::{ExpressionNode, Parser, Program};
    use crate::scanner::Scanner;
    use anyhow::Result;

    #[test]
    fn parse_empty_program() {
        let mut scanner = Scanner::new("");
        let mut parser = Parser::new(&mut scanner);
        let res = parser.parse().unwrap();
        assert_eq!(res.len(), 0)
    }

    #[test]
    fn parse_literal_outside_of_list() {
        let mut scanner = Scanner::new("true false");
        let mut parser = Parser::new(&mut scanner);

        if let Err(error) = parser.parse() {
            assert_eq!(
                error.to_string(),
                "[line 1] Error at 'true': Expected LeftParen, but get True"
            );
        } else {
            panic!("Parser must fail");
        };
    }

    #[test]
    fn parse_empty_list() {
        let mut scanner = Scanner::new("()");
        let mut parser = Parser::new(&mut scanner);

        let result = parser.parse().unwrap();

        // One empty list expression
        assert_eq!(*result, vec![vec![]]);
    }

    #[test]
    fn parse_bool_expression() {
        let mut scanner = Scanner::new("(true false)");
        let mut parser = Parser::new(&mut scanner);

        let result = parser.parse().unwrap();

        assert_eq!(
            *result,
            vec![vec![
                ExpressionNode::BooleanLiteral(true),
                ExpressionNode::BooleanLiteral(false)
            ]]
        );
    }

    #[test]
    fn parse_integer_numbers() {
        let mut scanner = Scanner::new("( -10 -1 0 1 2 42 1000 )");
        let mut parser = Parser::new(&mut scanner);

        let result = parser.parse().unwrap();

        assert_eq!(
            *result,
            vec![vec![
                ExpressionNode::IntegerNumberLiteral(-10),
                ExpressionNode::IntegerNumberLiteral(-1),
                ExpressionNode::IntegerNumberLiteral(0),
                ExpressionNode::IntegerNumberLiteral(1),
                ExpressionNode::IntegerNumberLiteral(2),
                ExpressionNode::IntegerNumberLiteral(42),
                ExpressionNode::IntegerNumberLiteral(1000),
            ]]
        );
    }

    #[test]
    fn parse_float_numbers() {
        let mut scanner = Scanner::new("( -10.0 -1.1 0.0 1.0 2.5 42.9999 1000.110111 )");
        let mut parser = Parser::new(&mut scanner);

        let result = parser.parse().unwrap();

        assert_eq!(
            *result,
            vec![vec![
                ExpressionNode::FloatNumberLiteral(-10.0),
                ExpressionNode::FloatNumberLiteral(-1.1),
                ExpressionNode::FloatNumberLiteral(0.0),
                ExpressionNode::FloatNumberLiteral(1.0),
                ExpressionNode::FloatNumberLiteral(2.5),
                ExpressionNode::FloatNumberLiteral(42.9999),
                ExpressionNode::FloatNumberLiteral(1000.110111),
            ]]
        );
    }

    #[test]
    fn parse_fraction_numbers() {
        let mut scanner = Scanner::new("( -1/2 1/2 0/1 1/33 )");
        let mut parser = Parser::new(&mut scanner);

        let result = parser.parse().unwrap();

        assert_eq!(
            *result,
            vec![vec![
                ExpressionNode::FractionNumberLiteral(-1, 2),
                ExpressionNode::FractionNumberLiteral(1, 2),
                ExpressionNode::FractionNumberLiteral(0, 1),
                ExpressionNode::FractionNumberLiteral(1, 33),
            ]]
        );
    }

    #[test]
    fn parse_string_literal() {
        let mut scanner = Scanner::new("( \"\" \"Hello world\" \"Meh\" )");
        let mut parser = Parser::new(&mut scanner);

        let result = parser.parse().unwrap();

        assert_eq!(
            *result,
            vec![vec![
                ExpressionNode::StringLiteral("".to_owned()),
                ExpressionNode::StringLiteral("Hello world".to_owned()),
                ExpressionNode::StringLiteral("Meh".to_owned()),
            ]]
        );
    }

    #[test]
    fn parse_identifier() {
        let mut scanner = Scanner::new("( x _x 'x x2 ?when do * / )");
        let mut parser = Parser::new(&mut scanner);

        let result = parser.parse().unwrap();

        assert_eq!(
            *result,
            vec![vec![
                ExpressionNode::Identifier("x".to_owned()),
                ExpressionNode::Identifier("_x".to_owned()),
                ExpressionNode::Identifier("'x".to_owned()),
                ExpressionNode::Identifier("x2".to_owned()),
                ExpressionNode::Identifier("?when".to_owned()),
                ExpressionNode::Identifier("do".to_owned()),
                ExpressionNode::Identifier("*".to_owned()),
                ExpressionNode::Identifier("/".to_owned()),
            ]]
        );
    }

    #[test]
    fn parse_keyword() {
        let mut scanner = Scanner::new("( :hello :12 :x1 :when)");
        let mut parser = Parser::new(&mut scanner);

        let result = parser.parse().unwrap();

        assert_eq!(
            *result,
            vec![vec![
                ExpressionNode::Keyword(":hello".to_owned()),
                ExpressionNode::Keyword(":12".to_owned()),
                ExpressionNode::Keyword(":x1".to_owned()),
                ExpressionNode::Keyword(":when".to_owned()),
            ]]
        );
    }

    #[test]
    fn parse_empty_array() {
        let mut scanner = Scanner::new("( [] )");
        let mut parser = Parser::new(&mut scanner);

        let result = parser.parse().unwrap();

        assert_eq!(*result, vec![vec![ExpressionNode::Array(vec![]),]]);
    }

    #[test]
    fn parse_empty_map() {
        let mut scanner = Scanner::new("( {} )");
        let mut parser = Parser::new(&mut scanner);

        let result = parser.parse().unwrap();

        assert_eq!(*result, vec![vec![ExpressionNode::Map(vec![]),]]);
    }

    #[test]
    fn parse_anonymous_function() {
        let mut scanner = Scanner::new("(#( + %1 2 ))");
        let mut parser = Parser::new(&mut scanner);

        let result = parser.parse().unwrap();

        assert_eq!(
            *result,
            vec![vec![ExpressionNode::AnonymousFunction(vec![
                ExpressionNode::Identifier("+".to_owned()),
                ExpressionNode::Identifier("%1".to_owned()),
                ExpressionNode::IntegerNumberLiteral(2)
            ]),]]
        );
    }

    #[test]
    fn parse_list_in_list() {
        let mut scanner = Scanner::new("(() ())");
        let mut parser = Parser::new(&mut scanner);

        let result = parser.parse().unwrap();

        assert_eq!(
            *result,
            vec![vec![
                ExpressionNode::FunctionCall(vec![]),
                ExpressionNode::FunctionCall(vec![])
            ]]
        );
    }
}
