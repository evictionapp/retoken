use syn::{parse::Parse, Ident, LitStr, Visibility};

#[derive(Debug)]
pub struct ReStmt {
    pub vis: Visibility,
    pub skip: bool,
    pub ident: Ident,
    pub pat: LitStr,
}

impl Parse for ReStmt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let pat: LitStr = input.parse()?;
        let pat_value = pat.value();

        if pat_value.starts_with('^') {
            return Err(syn::Error::new(
                pat.span(),
                r#"regexes get "^" prepended automatically"#,
            ));
        }

        let new_pat = format!("^{}", pat.value());
        let pat = LitStr::new(new_pat.as_str(), pat.span());

        Ok(Self {
            vis: Visibility::Inherited,
            ident,
            pat,
            skip: false,
        })
    }
}

impl ReStmt {
    pub fn into_skip(self) -> Self {
        Self { skip: true, ..self }
    }
}
