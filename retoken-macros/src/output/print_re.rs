use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::input::re_stmt::ReStmt;

pub fn print_re(re_stmt: &ReStmt, alphabet_ident: &Ident, skip: Option<&Ident>) -> TokenStream {
    let vis = &re_stmt.vis;
    let ident = &re_stmt.ident;
    let pat = &re_stmt.pat;

    let skip = match skip {
        Some(skip_id) => quote!(let _ = tokenizer.skip::<#skip_id>();),
        None => TokenStream::new(),
    };

    quote! {
        #[derive(Debug, Clone)]
        #vis struct #ident<'a> {
            content: &'a str,
            span: retoken::Span,
        }

        impl<'a> retoken::Token<'a, #alphabet_ident> for #ident<'a> {
            fn content(&'a self) -> &'a str {
                self.content
            }

            fn span(&'a self) -> retoken::Span {
                self.span.clone()
            }

            fn peek(tokenizer: &retoken::Tokenizer) -> bool {
                #skip

                use retoken::lazy_regex;
                let re = lazy_regex::regex!(#pat);
                tokenizer.peek_re(re)
            }

            fn token(tokenizer: &'a retoken::Tokenizer) -> retoken::Result<Self, #alphabet_ident> {
                #skip

                use retoken::lazy_regex;
                let re = lazy_regex::regex!(#pat);

                match tokenizer.token_re(re)? {
                    Some((content, span)) => Ok(Self {
                        content,
                        span,
                    }),
                    None => Err(retoken::Error::ExpectedToken(retoken::ExpectedTokenError{
                        cursor: tokenizer.cursor(),
                        token: #alphabet_ident :: #ident,
                    }))
                }
            }
        }
    }
}
