use thiserror::Error;

use crate::span::Span;

#[derive(Debug, Error)]
#[error("failed to slice at {0}")]
pub struct SliceError(pub usize);

#[derive(Debug, Error)]
#[error("expected token {token:?} at {cursor}")]
pub struct ExpectedTokenError<Alphabet> {
    pub cursor: usize,
    pub token: Alphabet,
}

#[derive(Debug, Error)]
pub enum Error<Alphabet> {
    #[error("{0}")]
    Slice(#[from] SliceError),
    #[error("{0}")]
    ExpectedToken(ExpectedTokenError<Alphabet>),
}

impl<Alphabet> Error<Alphabet> {
    pub fn slice(cursor: usize) -> Self {
        Self::Slice(SliceError(cursor))
    }

    pub fn expected_token(cursor: usize, token: Alphabet) -> Self {
        Self::ExpectedToken(ExpectedTokenError { cursor, token })
    }

    pub fn span(&self) -> Span {
        let start = match &self {
            Self::Slice(slice_error) => slice_error.0,
            Self::ExpectedToken(expected_token_error) => expected_token_error.cursor,
        };

        Span { start, end: start }
    }
}
