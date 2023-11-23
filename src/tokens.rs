#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Integer<'source> {
    Decimal(&'source str),
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Literal<'source> {
    Integer(Integer<'source>),
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Int,
    Return,
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

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Identifier<'source>(&'source [u8]);

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Token<'source> {
    Keyword(Keyword),
    Literal(Literal<'source>),
    Pair(Pair),
    Identifier(Identifier<'source>),
}

use once_cell::sync::Lazy;
use regex::bytes::Regex;

pub fn tokenize<'source>(source: &'source [u8]) -> Vec<Token<'source>> {
    static OPEN_BRACE: Lazy<Regex> = Lazy::new(|| Regex::new("{").unwrap());
    static CLOSE_BRACE: Lazy<Regex> = Lazy::new(|| Regex::new("}").unwrap());
    static OPEN_PAR: Lazy<Regex> = Lazy::new(|| Regex::new(r"\(").unwrap());
    static CLOSE_PAR: Lazy<Regex> = Lazy::new(|| Regex::new(r"\)").unwrap());
    static SEMICOLON: Lazy<Regex> = Lazy::new(|| Regex::new(r";").unwrap());
    static KEYWORD_INT: Lazy<Regex> = Lazy::new(|| Regex::new(r"int").unwrap());
    static KEYWORD_RETURN: Lazy<Regex> = Lazy::new(|| Regex::new(r"return").unwrap());
    static IDENTIFIER: Lazy<Regex> = Lazy::new(|| Regex::new(r"[a-zA-Z]\w*").unwrap());
}
