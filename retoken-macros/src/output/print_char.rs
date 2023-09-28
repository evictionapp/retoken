use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::input::char_stmt::CharStmt;

pub fn print_char(
    char_stmt: &CharStmt,
    alphabet_ident: &Ident,
    skip: Option<&Ident>,
) -> TokenStream {
    let vis = &char_stmt.vis;
    let ident = &char_stmt.ident;

    let ch = &char_stmt.pat;
    let ch_static_str = char_stmt.pat.value().to_string();

    let skip = match skip {
        Some(skip_id) => quote!(let _ = tokenizer.skip::<#skip_id>();),
        None => TokenStream::new(),
    };

    quote! {
        #[derive(Debug, Clone)]
        #vis struct #ident(retoken::Span);

        impl retoken::Char for #ident {
            fn ch() -> char {
                #ch
            }
        }

        impl<'a> retoken::Token<'a, #alphabet_ident> for #ident {
            fn content(&'a self) -> &'a str {
                #ch_static_str
            }

            fn span(&'a self) -> retoken::Span {

                self.0.clone()
            }

            fn peek(tokenizer: &retoken::Tokenizer) -> bool {
                #skip

                tokenizer.peek_char(#ch)
            }

            fn token(tokenizer: &retoken::Tokenizer) -> retoken::Result<Self, #alphabet_ident> {
                #skip

                match tokenizer.token_char(#ch)? {
                    Some(span) => Ok(Self(span)),
                    _ => Err(retoken::Error::ExpectedToken(retoken::ExpectedTokenError{
                        cursor: tokenizer.cursor(),
                        token: #alphabet_ident :: #ident,
                    }))
                }
            }
        }
    }
}
