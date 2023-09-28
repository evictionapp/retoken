use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use super::{char_stmt::CharStmt, re_stmt::ReStmt};

#[derive(Debug)]
pub enum TokenStmt {
    Char(CharStmt),
    Re(ReStmt),
    Custom(Ident),
    CustomLifetime(Ident),
}

impl TokenStmt {
    pub fn ident(&self) -> &Ident {
        match self {
            Self::Char(ch) => &ch.ident,
            Self::Re(re) => &re.ident,
            Self::Custom(id) | Self::CustomLifetime(id) => id,
        }
    }

    pub fn has_lifetime(&self) -> bool {
        matches!(self, Self::Re(_) | Self::CustomLifetime(_))
    }

    pub fn grammar_variant(&self) -> TokenStream {
        let ident = self.ident();
        match self.has_lifetime() {
            true => quote!(#ident( #ident <'a> )),
            false => quote!(#ident ( #ident ) ),
        }
    }
}
