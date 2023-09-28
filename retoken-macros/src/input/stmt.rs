use proc_macro2::Span;
use syn::{parse::Parse, Visibility};

use super::{re_stmt::ReStmt, stmt_type::StmtType, token_stmt::TokenStmt, vis_ident::VisIdent};

pub struct Stmt {
    pub span: Span,
    pub vis: Visibility,
    pub stmt_type: StmtType,
}

impl TryInto<TokenStmt> for Stmt {
    type Error = syn::Error;

    fn try_into(self) -> Result<TokenStmt, Self::Error> {
        match self.stmt_type {
            StmtType::Token(token_stmt) => Ok(token_stmt),
            StmtType::Alphabet(id) | StmtType::Grammar(id) => Err(
                syn::Error::new(
                    id.span(),
                    "expected only token statements. alphabet and grammar statements are only allowed at the top"
                 )

            )
        }
    }
}

impl Parse for Stmt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let span = input.span();
        let vis: Visibility = input.parse()?;

        let stmt_type = input.parse::<StmtType>()?.with_visability(vis.clone());

        Ok(Self {
            span,
            vis,
            stmt_type,
        })
    }
}

impl Stmt {
    pub fn into_alphabet(self) -> Option<VisIdent> {
        match self.stmt_type {
            StmtType::Alphabet(ident) => Some(VisIdent {
                ident,
                vis: self.vis,
            }),
            _ => None,
        }
    }

    pub fn into_grammar(self) -> Option<VisIdent> {
        match self.stmt_type {
            StmtType::Grammar(ident) => Some(VisIdent {
                ident,
                vis: self.vis,
            }),
            _ => None,
        }
    }

    pub fn into_re(self) -> Option<ReStmt> {
        let token_stmt = match self.stmt_type {
            StmtType::Token(token_stmt) => token_stmt,
            _ => return None,
        };

        match token_stmt {
            TokenStmt::Re(re) => Some(re),
            _ => None,
        }
    }

    pub fn is_skip(&self) -> bool {
        let token_stmt = match &self.stmt_type {
            StmtType::Token(token_stmt) => token_stmt,
            _ => return false,
        };

        match token_stmt {
            TokenStmt::Re(re) => re.skip,
            _ => false,
        }
    }
}
