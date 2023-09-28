use quote::{ToTokens, TokenStreamExt};

use crate::input::ast::Ast;

use self::{alphagrammar::AlphaGrammar, print_skip::print_skip, token_structs::TokenStructs};

pub mod alphagrammar;
pub mod print_char;
pub mod print_re;
pub mod print_skip;
pub mod token_structs;

pub struct Output {
    ast: Ast,
}

impl Output {
    pub fn new(ast: Ast) -> syn::Result<Self> {
        if ast.tokens.is_empty() {
            return Err(syn::Error::new(
                ast.grammar.ident.span(),
                "expected at least one token statement",
            ));
        }

        Ok(Self { ast })
    }
}

impl ToTokens for Output {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ag = AlphaGrammar {
            alphabet: &self.ast.alphabet,
            grammar: &self.ast.grammar,
            token_stmts: &self.ast.tokens,
        };

        let token_structs = TokenStructs {
            alphabet_ident: &self.ast.alphabet.ident,
            token_stmts: &self.ast.tokens,
            skip: self.ast.skip.as_ref().map(|el| &el.ident),
        };

        if let Some(skip) = self.ast.skip.as_ref() {
            tokens.append_all(print_skip(skip));
        }

        tokens.append_all(token_structs.into_token_stream());

        tokens.append_all(ag.into_token_stream());
    }
}
