use crate::ast::*;
use crate::token::*;
use std::slice::Iter;
use thiserror::Error;
use colored::*;
use std::collections::HashMap;

pub type ParseResult<T> = Result<T, ParseError>;

pub struct Parser<'p> {
    tokens: Iter<'p, Token>,
    current: Token,
    peek: Token,
}

impl<'p> Parser<'p> {
    pub fn new(tokens: Iter<'p, Token>) -> Self {
        Parser {
            tokens: tokens,
            current: Token::Eof,
            peek: Token::Eof,
        }
    }

    pub fn parse(&mut self) -> ParseResult<AstNode> {
        match self.current.clone() {
            Token::LBrace => self.parse_key_value_list(),
            Token::LBracket => self.parse_list(),
            Token::String(s) => self.parse_string(s),
            Token::Integer(i) => self.parse_integer(i),
            Token::Float(f) => self.parse_float(f),
            Token::Identifier(s) => self.parse_identifier(s),
            _ => { println!("{}\n", self.current); return Err(ParseError::UnexpectedToken(self.current.clone())) },
        }
    }

    fn parse_key_value_list(&mut self) -> ParseResult<AstNode> {
        // current: LBrace
        self.expect_token_and_read(Token::LBrace)?;
        
        let mut value = Vec::<AstNode>::new();
        let mut map = HashMap::<String, AstNode>::new();

        if self.current != Token::RBrace {
            let kv = self.parse_key_value()?;
            value.push(kv.clone());

            let kv = kv.to_kv();
            map.insert(kv.0, kv.1);

            if self.current == Token::Comma {
                self.expect_token_and_read(Token::Comma)?;
            }

            while self.current_is(Token::String("".to_string()))
                || self.current_is(Token::Integer(0))
                || self.current_is(Token::Float(0.0)) {
                let kv = self.parse_key_value()?;
                value.push(kv.clone());
                
                let kv = kv.to_kv();
                map.insert(kv.0, kv.1);

                if self.current_is(Token::Comma) {
                    self.expect_token_and_read(Token::Comma)?;
                }

                if self.current_is(Token::RBrace) {
                    break;
                }
            }

            self.expect_token_and_read(Token::RBrace)?;
        }

        Ok(AstNode::KeyValueList { value, map })
    }

    fn parse_key_value(&mut self) -> ParseResult<AstNode> {
        let key = self.current.clone().into();

        self.expect_token_and_read(Token::String("".to_string()))?;
        self.expect_token_and_read(Token::Colon)?;

        let value = self.parse()?;

        Ok(AstNode::KeyValue { key, value: Box::new(value) })
    }

    fn parse_list(&mut self) -> ParseResult<AstNode> {
        let mut value = Vec::<AstNode>::new();

        self.expect_token_and_read(Token::LBracket)?;

        if self.current_is(Token::RBracket) {
            self.expect_token_and_read(Token::RBracket)?;
            return Ok(AstNode::List { value });
        }

        value.push(self.parse()?);

        while self.current_is(Token::Comma) {
            self.expect_token_and_read(Token::Comma)?;

            value.push(self.parse()?);
        }

        self.expect_token_and_read(Token::RBracket)?;

        Ok(AstNode::List { value })
    }

    fn parse_string(&mut self, s: String) -> ParseResult<AstNode> {
        self.expect_token_and_read(Token::String("".to_string()))?;

        Ok(AstNode::String { value: s })
    }

    fn parse_identifier(&mut self, s: String) -> ParseResult<AstNode> {
        self.expect_identifier_and_read()?;

        Ok(AstNode::String { value: s })
    }

    fn parse_integer(&mut self, i: i64) -> ParseResult<AstNode> {
        self.expect_token_and_read(Token::Integer(0))?;

        Ok(AstNode::Integer { value: i })
    }

    fn parse_float(&mut self, f: f64) -> ParseResult<AstNode> {
        self.expect_token_and_read(Token::Float(0.0))?;

        Ok(AstNode::Float { value: f })
    }

    fn expect_token(&mut self, token: Token) -> ParseResult<Token> {
        if self.current_is(token) {
            Ok(self.current.clone())
        } else {
            Err(ParseError::UnexpectedToken(self.current.clone()))
        }
    }

    fn expect_token_and_read(&mut self, token: Token) -> ParseResult<Token> {
        let result = self.expect_token(token)?;

        self.read();

        Ok(result)
    }

    fn expect_identifier_and_read(&mut self) -> ParseResult<Token> {
        self.expect_token_and_read(Token::Identifier("".to_string()))
    }

    fn current_is(&self, token: Token) -> bool {
        std::mem::discriminant(&self.current) == std::mem::discriminant(&token)
    }

    fn read(&mut self) {
        self.current = self.peek.clone();
        self.peek = if let Some(token) = self.tokens.next() { token.clone() } else { Token::Eof };
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected token {0:?}")]
    UnexpectedToken(Token),
}

impl ParseError {
    pub fn print(self) {
        eprintln!("{}", format!("{}", self).red().bold());
    }
}