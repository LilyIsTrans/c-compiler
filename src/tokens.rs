use std::fmt::{write, Display, Write};

use crate::common::BasicText;
use log::warn;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Integer<'source> {
    Decimal(&'source BasicText),
}

impl<'source> Display for Integer<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(not(feature = "unicode"))]
            Integer::Decimal(string) => string.escape_ascii().fmt(f),
            #[cfg(feature = "unicode")]
            Integer::Decimal(string) => string.fmt(f),
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Literal<'source> {
    Integer(Integer<'source>),
}

impl<'source> Display for Literal<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Integer(i) => i.fmt(f),
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Int,
    Return,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Int => write!(f, "int"),
            Keyword::Return => write!(f, "return"),
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum PairSide {
    Opening,
    Closing,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Pair {
    Parentheses(PairSide),
    Brackets(PairSide),
    Braces(PairSide),
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pair::Parentheses(PairSide::Opening) => write!(f, "("),
            Pair::Brackets(PairSide::Opening) => write!(f, "["),
            Pair::Braces(PairSide::Opening) => write!(f, "{{"),
            Pair::Parentheses(PairSide::Closing) => write!(f, ")"),
            Pair::Brackets(PairSide::Closing) => write!(f, "]"),
            Pair::Braces(PairSide::Closing) => write!(f, "}}"),
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Identifier<'source>(&'source BasicText);

impl<'source> Display for Identifier<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(not(feature = "unicode"))]
        return self.0.escape_ascii().fmt(f);
        #[cfg(feature = "unicode")]
        self.0.fmt(f)
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Token<'source> {
    Keyword(Keyword),
    Literal(Literal<'source>),
    Pair(Pair),
    Identifier(Identifier<'source>),
    Semicolon,
}

impl<'source> Display for Token<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Keyword(t) => t.fmt(f),
            Token::Literal(t) => t.fmt(f),
            Token::Pair(t) => t.fmt(f),
            Token::Identifier(t) => t.fmt(f),
            Token::Semicolon => write!(f, ";"),
        }
    }
}

impl<'source> From<Keyword> for Token<'source> {
    fn from(value: Keyword) -> Self {
        Self::Keyword(value)
    }
}

impl<'source> From<Literal<'source>> for Token<'source> {
    fn from(value: Literal<'source>) -> Self {
        Self::Literal(value)
    }
}

impl<'source> From<Pair> for Token<'source> {
    fn from(value: Pair) -> Self {
        Self::Pair(value)
    }
}

impl<'source> From<Identifier<'source>> for Token<'source> {
    fn from(value: Identifier<'source>) -> Self {
        Self::Identifier(value)
    }
}

use once_cell::sync::Lazy;

#[cfg(not(feature = "unicode"))]
use regex::bytes::Regex;
#[cfg(feature = "unicode")]
use regex::Regex;

pub fn tokenize<'source>(source: &'source BasicText) -> Vec<Token<'source>> {
    static OPEN_BRACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A\{").unwrap());
    static CLOSE_BRACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A\}").unwrap());
    static OPEN_PAR: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A\(").unwrap());
    static CLOSE_PAR: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A\)").unwrap());
    static SEMICOLON: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A;").unwrap());
    static KEYWORD_INT: Lazy<Regex> = Lazy::new(|| Regex::new(r"\Aint").unwrap());
    static KEYWORD_RETURN: Lazy<Regex> = Lazy::new(|| Regex::new(r"\Areturn").unwrap());
    #[cfg(feature = "unicode")]
    static IDENTIFIER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A[a-zA-Z]\w*").unwrap());
    #[cfg(not(feature = "unicode"))]
    static IDENTIFIER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A[a-zA-Z](?-u:\w)*").unwrap());
    static DECIMAL_INTEGER_LITERAL: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A[0-9]+").unwrap());
    #[cfg(feature = "unicode")]
    static WHITESPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A\s+").unwrap());
    #[cfg(not(feature = "unicode"))]
    static WHITESPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A(?-u:\s)+").unwrap());

    let mut stream: Vec<Token<'source>> = Vec::with_capacity(source.len());

    let mut text: &'source BasicText = source;

    while !text.is_empty() {
        if let Some(mat) = KEYWORD_INT.find(text) {
            stream.push(Keyword::Int.into());
            text = &text[mat.end()..];
        } else if let Some(mat) = KEYWORD_RETURN.find(text) {
            stream.push(Keyword::Return.into());
            text = &text[mat.end()..];
        } else if let Some(mat) = OPEN_BRACE.find(text) {
            stream.push(Pair::Braces(PairSide::Opening).into());
            text = &text[mat.end()..];
        } else if let Some(mat) = CLOSE_BRACE.find(text) {
            stream.push(Pair::Braces(PairSide::Closing).into());
            text = &text[mat.end()..];
        } else if let Some(mat) = OPEN_PAR.find(text) {
            stream.push(Pair::Parentheses(PairSide::Opening).into());
            text = &text[mat.end()..];
        } else if let Some(mat) = CLOSE_PAR.find(text) {
            stream.push(Pair::Parentheses(PairSide::Closing).into());
            text = &text[mat.end()..];
        } else if let Some(mat) = SEMICOLON.find(text) {
            stream.push(Token::Semicolon);
            text = &text[mat.end()..];
        } else if let Some(mat) = WHITESPACE.find(text) {
            text = &text[mat.end()..];
        } else if let Some(mat) = DECIMAL_INTEGER_LITERAL.find(text) {
            #[cfg(not(feature = "unicode"))]
            stream.push(Literal::Integer(Integer::Decimal(mat.as_bytes())).into());
            #[cfg(feature = "unicode")]
            stream.push(Literal::Integer(Integer::Decimal(mat.as_str())).into());
            text = &text[mat.end()..];
        } else if let Some(mat) = IDENTIFIER.find(text) {
            #[cfg(not(feature = "unicode"))]
            stream.push(Identifier(mat.as_bytes()).into());
            #[cfg(feature = "unicode")]
            stream.push(Identifier(mat.as_str()).into());
            text = &text[mat.end()..];
        } else {
            warn!("Unrecognized Token: {}!", text[0]);
            text = &text[1..];
        }
    }

    stream
}
