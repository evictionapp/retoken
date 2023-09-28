use syn::parse::Parse;

use super::{re_stmt::ReStmt, stmt_list::StmtList, token_stmt::TokenStmt, vis_ident::VisIdent};

#[derive(Debug)]
pub struct Ast {
    pub alphabet: VisIdent,
    pub grammar: VisIdent,
    pub skip: Option<ReStmt>,
    pub tokens: Vec<TokenStmt>,
}

impl Parse for Ast {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut stmt_list: StmtList = input.parse()?;

        let alphabet = stmt_list.stmts.pop_front().ok_or(syn::Error::new(
            stmt_list.span,
            "the first declaration must be an alphabet declaration",
        ))?;
        let alphabet_span = alphabet.span;
        let alphabet = alphabet.into_alphabet().ok_or(syn::Error::new(
            alphabet_span,
            "the first declaration must be an alphabet declaration",
        ))?;

        let grammar = stmt_list.stmts.pop_front().ok_or(syn::Error::new(
            stmt_list.span,
            "the second declaration must be an grammar declaration",
        ))?;
        let grammar_span = grammar.span;
        let grammar = grammar.into_grammar().ok_or(syn::Error::new(
            grammar_span,
            "the second declaration must be an grammar declaration",
        ))?;

        let skip = stmt_list.skip()?;
        let tokens = stmt_list.tokens()?;

        Ok(Self {
            alphabet,
            grammar,
            skip,
            tokens,
        })
    }
}
