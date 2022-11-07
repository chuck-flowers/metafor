use core::ops::Index;
use proc_macro2::Span as Span2;
use std::collections::HashMap;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result as ParseResult;
use syn::spanned::Spanned;
use syn::Ident;
use syn::Token;

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct StructValue(HashMap<Ident, Ident>);

impl StructValue {
    pub fn fields<'a>(&'a self) -> impl Iterator<Item = (&'a Ident, &'a Ident)> {
        self.0.iter()
    }
}

impl Index<&Ident> for StructValue {
    type Output = Ident;
    fn index(&self, index: &Ident) -> &Self::Output {
        &self.0[index]
    }
}

impl Parse for StructValue {
    fn parse(input: ParseStream) -> ParseResult<Self> {
		let content;
		syn::braced!(content in input);

        let mut fields = HashMap::new();

        loop {
            let ident: Ident = content.parse()?;
            let _: Token!(:) = content.parse()?;
            let ident_val: Ident = content.parse()?;

            fields.insert(ident, ident_val);

            if content.is_empty() {
                break;
            }

            let _: Token!(,) = content.parse()?;
        }

        Ok(Self(fields))
    }
}

impl Spanned for StructValue {
    fn span(&self) -> Span2 {
        self.0.iter().next().unwrap().0.span()
    }
}
