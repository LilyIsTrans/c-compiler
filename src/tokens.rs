use std::fmt::Display;

use crate::common::BasicText;

use tracing::instrument;
use tracing::warn;

use once_cell::sync::Lazy;

#[cfg(not(feature = "unicode"))]
use regex::bytes::Regex;
#[cfg(feature = "unicode")]
use regex::Regex;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum IntegerKind {
    Decimal,
    Hexadecimal,
    Binary,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Integer<'source> {
    source: &'source BasicText,
    kind: IntegerKind,
}

static DECIMAL_INTEGER_LITERAL: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A(0d)?[0-9]+").unwrap());
static HEXADECIMAL_INTEGER_LITERAL: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\A0x[0-9&&a-f&&A-F]+").unwrap());
static BINARY_INTEGER_LITERAL: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A0b[01]+").unwrap());

impl<'source> Display for Integer<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(not(feature = "unicode"))]
        {
            self.source.escape_ascii().fmt(f)
        }
        #[cfg(feature = "unicode")]
        {
            self.source.fmt(f)
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

static KEYWORD_INT: Lazy<Regex> = Lazy::new(|| Regex::new(r"\Aint").unwrap());
static KEYWORD_RETURN: Lazy<Regex> = Lazy::new(|| Regex::new(r"\Areturn").unwrap());

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

static OPEN_BRACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A\{").unwrap());
static CLOSE_BRACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A\}").unwrap());
static OPEN_PAR: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A\(").unwrap());
static CLOSE_PAR: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A\)").unwrap());

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

#[cfg(feature = "unicode")]
static IDENTIFIER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A[a-zA-Z]\w*").unwrap());
#[cfg(not(feature = "unicode"))]
static IDENTIFIER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A[a-zA-Z](?-u:\w)*").unwrap());

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

static SEMICOLON: Lazy<Regex> = Lazy::new(|| Regex::new(r"\A;").unwrap());

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

pub type TokenStream<'source> = Vec<Token<'source>>;

#[instrument]
pub fn tokenize<'source>(source: &'source BasicText) -> TokenStream<'source> {
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
            stream.push(
                Literal::Integer(Integer {
                    kind: IntegerKind::Decimal,
                    source: mat.as_bytes(),
                })
                .into(),
            );
            #[cfg(feature = "unicode")]
            stream.push(
                Literal::Integer(Integer {
                    kind: IntegerKind::Decimal,
                    source: mat.as_str(),
                })
                .into(),
            );
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
