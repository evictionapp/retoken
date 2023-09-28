use proc_macro2::TokenStream;
use quote::quote;

use crate::input::re_stmt::ReStmt;

pub fn print_skip(re_stmt: &ReStmt) -> TokenStream {
    let vis = &re_stmt.vis;
    let ident = &re_stmt.ident;
    let pat = &re_stmt.pat;

    quote! {
        #[derive(Debug, Clone)]
        #vis struct #ident;

        impl retoken::Skip for #ident {
            fn skip(tokenizer: &retoken::Tokenizer) -> bool {
                use retoken::lazy_regex;
                let re = lazy_regex::regex!(#pat);

                match tokenizer.token_re(re) {
                    Ok(Some(_)) => true,
                    _ => false,
                }
            }
        }
    }
}
