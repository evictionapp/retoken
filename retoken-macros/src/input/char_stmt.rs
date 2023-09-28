use syn::{parse::Parse, Ident, LitChar, Visibility};

#[derive(Debug)]
pub struct CharStmt {
    pub vis: Visibility,
    pub ident: Ident,
    pub pat: LitChar,
}

impl Parse for CharStmt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let pat = input.parse()?;

        Ok(Self {
            vis: Visibility::Inherited,
            ident,
            pat,
        })
    }
}
