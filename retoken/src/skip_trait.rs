use crate::Tokenizer;

pub trait Skip {
    fn skip(tokenizer: &Tokenizer) -> bool;
}
