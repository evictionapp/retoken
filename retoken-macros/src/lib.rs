use input::ast::Ast;
use output::Output;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

mod input;
mod output;

#[proc_macro]
pub fn relex(input_tokens: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input_tokens as Ast);

    let output = match Output::new(ast) {
        Ok(ok) => ok,
        Err(err) => return err.into_compile_error().into(),
    };

    output.into_token_stream().into()
}
