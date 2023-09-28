use syn::{parse::Parse, Ident, Visibility};

use super::{char_stmt::CharStmt, re_stmt::ReStmt, token_stmt::TokenStmt};

pub enum StmtType {
    Alphabet(Ident),
    Grammar(Ident),
    Token(TokenStmt),
}

impl Parse for StmtType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let stmt_type_ident: Ident = input.parse()?;
        let ok = match stmt_type_ident.to_string().as_str() {
            "alphabet" => StmtType::Alphabet(input.parse()?),
            "grammar" => StmtType::Grammar(input.parse()?),
            "char" => StmtType::Token(TokenStmt::Char(input.parse()?)),
            "re" => StmtType::Token(TokenStmt::Re(input.parse()?)),
            "skip" => StmtType::Token(TokenStmt::Re(ReStmt::into_skip(input.parse()?))),
            "owned" => StmtType::Token(TokenStmt::Custom(input.parse()?)),
            "borrowed" => StmtType::Token(TokenStmt::CustomLifetime(input.parse()?)),
            _ => {
                return Err(syn::Error::new(
                    stmt_type_ident.span(),
                    format!(
                        r#"expected "alphabet" / "grammar" / "char" / "re" / "skip" / "owned" / "borrowed" got "{}""#,
                        stmt_type_ident
                    ),
                ))
            }
        };

        Ok(ok)
    }
}

impl StmtType {
    pub fn with_visability(self, vis: Visibility) -> Self {
        match self {
            Self::Token(token_stmt) => match token_stmt {
                TokenStmt::Char(ch) => Self::Token(TokenStmt::Char(CharStmt { vis, ..ch })),
                TokenStmt::Re(re) => Self::Token(TokenStmt::Re(ReStmt { vis, ..re })),
                TokenStmt::Custom(c) => Self::Token(TokenStmt::Custom(c)),
                TokenStmt::CustomLifetime(c) => Self::Token(TokenStmt::CustomLifetime(c)),
            },
            _ => self,
        }
    }
}
