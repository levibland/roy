use logos::{Lexer, Logos};
use std::fmt;

#[derive(Debug, Clone, Logos, PartialEq)]
pub enum Token {
    #[regex(r"[a-zA-Z_]+", to_string)]
    Identifier(String),
    #[regex(r##""(?:[^"\\]|\\.)*""##, to_string)]
    String(String),
    #[regex(r"[0-9]+", to_int, priority = 2)]
    Integer(i64),
    #[regex(r"([0-9]+[.])?[0-9]+", to_float)]
    Float(f64),

    #[token(",")]
    Comma,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(":")]
    Colon,

    Eof,

    #[error]
    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(s) => write!(f, "{}", s),
            Self::String(s) => write!(f, "{}", s),
            Self::Integer(i) => write!(f, "{}", i),
            Self::Float(n) => write!(f, "{}", n),
            Self::Comma => write!(f, ","),
            Self::LBrace => write!(f, "{{"),
            Self::RBrace => write!(f, "}}"),
            Self::LBracket => write!(f, "["),
            Self::RBracket => write!(f, "]"),
            Self::Colon => write!(f, ":"),
            Self::Eof => write!(f, "\0"),
            Self::Error => write!(f, "error"),
        }
    }
}

impl Into<String> for Token {
    fn into(self) -> String {
        match self {
            Self::Identifier(s) => s,
            Self::String(s) => s,
            _ => unreachable!(),
        }
    }
}

fn to_string(lex: &mut Lexer<Token>) -> Option<String> {
    let mut str = lex.slice().to_string();

    if str.starts_with("\"") {
        str.remove(0);
    }

    if str.ends_with('"') {
        str.remove(str.len() - 1);
    }

    Some(str)
}

fn to_int(lex: &mut Lexer<Token>) -> Option<i64> {
    Some(lex.slice().parse().ok()?)
}

fn to_float(lex: &mut Lexer<Token>) -> Option<f64> {
    Some(lex.slice().parse().ok()?)
}