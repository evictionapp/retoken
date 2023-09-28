use syn::{Ident, Visibility};

#[derive(Debug)]
pub struct VisIdent {
    pub vis: Visibility,
    pub ident: Ident,
}
