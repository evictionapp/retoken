mod char_trait;
mod error;
mod skip_trait;
mod span;
mod token_trait;
mod tokenizer;

pub use char_trait::Char;
pub use error::{Error, ExpectedTokenError, SliceError};
pub use lazy_regex;
pub use retoken_macros::relex;
pub use skip_trait::Skip;
pub use span::Span;
pub use token_trait::Token;
pub use tokenizer::Tokenizer;
pub type Result<T, Alphabet> = std::result::Result<T, Error<Alphabet>>;
