use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::Ident;

use crate::input::token_stmt::TokenStmt;

use super::{print_char::print_char, print_re::print_re};

pub struct TokenStructs<'a> {
    pub alphabet_ident: &'a Ident,
    pub token_stmts: &'a [TokenStmt],
    pub skip: Option<&'a Ident>,
}

impl<'a> ToTokens for TokenStructs<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for token_stmt in self.token_stmts {
            let output = match token_stmt {
                TokenStmt::Char(ch) => print_char(ch, self.alphabet_ident, self.skip),
                TokenStmt::Re(re) => print_re(re, self.alphabet_ident, self.skip),
                _ => TokenStream::new(),
            };

            tokens.append_all(output);
        }
    }
}
