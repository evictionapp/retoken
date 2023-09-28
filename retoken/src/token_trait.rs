use crate::{Result, Span, Tokenizer};

pub trait Token<'a, Alphabet>: Sized {
    fn content(&'a self) -> &'a str;

    fn span(&'a self) -> Span;

    fn token(tokenizer: &'a Tokenizer) -> Result<Self, Alphabet>;

    fn peek(tokenizer: &'a Tokenizer) -> bool;
}
