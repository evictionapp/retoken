use retoken::{Token, Tokenizer};

use crate::lexical::{
    CloseAngle, CloseBracket, Equal, Ident, OpenAngle, OpenBracket, Quoted, Slash,
};

mod lexical;

fn main() {
    let foo_ml = include_str!("../../foo.fooml");

    let tokenizer = Tokenizer::new(foo_ml);

    println!("{:?}", OpenAngle::token(&tokenizer));
    println!("{:?}", Ident::token(&tokenizer));
    println!("{:?}", Ident::token(&tokenizer));
    println!("{:?}", Equal::token(&tokenizer));
    println!("{:?}", Quoted::token(&tokenizer));
    println!("{:?}", Ident::token(&tokenizer));
    println!("{:?}", Equal::token(&tokenizer));
    println!("{:?}", OpenBracket::token(&tokenizer));
    println!("{:?}", Ident::token(&tokenizer));
    println!("{:?}", CloseBracket::token(&tokenizer));
    println!("{:?}", Slash::token(&tokenizer));
    println!("{:?}", CloseAngle::token(&tokenizer));
}
