use std::collections::VecDeque;

use proc_macro2::Span;
use syn::parse::Parse;

use super::{re_stmt::ReStmt, stmt::Stmt, token_stmt::TokenStmt};

pub struct StmtList {
    pub stmts: VecDeque<Stmt>,
    pub span: Span,
}

impl Parse for StmtList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let mut stmts = VecDeque::new();

        while !input.is_empty() {
            stmts.push_back(input.parse()?)
        }

        Ok(Self { stmts, span })
    }
}

impl StmtList {
    pub fn skip(&mut self) -> syn::Result<Option<ReStmt>> {
        let skip_els: Vec<&Stmt> = self.stmts.iter().filter(|el| el.is_skip()).collect();
        if let Some(el) = skip_els.get(1) {
            return Err(syn::Error::new(el.span, "expected only one skip element"));
        }
        let skip_idx = self.stmts.iter().position(|el| el.is_skip());

        match skip_idx {
            Some(skip_idx) => Ok(self.stmts.remove(skip_idx).and_then(|el| el.into_re())),
            None => Ok(None),
        }
    }

    pub fn tokens(self) -> syn::Result<Vec<TokenStmt>> {
        self.stmts.into_iter().map(|el| el.try_into()).collect()
    }
}
