use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

use crate::input::{token_stmt::TokenStmt, vis_ident::VisIdent};

pub struct AlphaGrammar<'a> {
    pub alphabet: &'a VisIdent,
    pub grammar: &'a VisIdent,
    pub token_stmts: &'a [TokenStmt],
}

impl<'a> ToTokens for AlphaGrammar<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let alphabet_ident = &self.alphabet.ident;
        let grammar_ident = &self.grammar.ident;
        let alphabet_vis = &self.alphabet.vis;
        let grammar_vis = &self.grammar.vis;

        let idents_no_lifetime = self.token_stmts.iter().map(|stmt| stmt.ident());
        let grammar_variants = self.token_stmts.iter().map(|stmt| stmt.grammar_variant());

        let lifetime = match self.token_stmts.iter().any(|el| el.has_lifetime()) {
            true => quote!(<'a>),
            false => TokenStream::new(),
        };

        let output = quote! {
            #[derive(Debug, Clone, Copy)]
            #alphabet_vis enum #alphabet_ident {
                #(#idents_no_lifetime),*
            }

            #[derive(Debug, Clone)]
            #grammar_vis enum #grammar_ident #lifetime {
                #(#grammar_variants),*
            }
        };

        tokens.append_all(output);
    }
}
