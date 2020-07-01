use core::ops::Index;
use proc_macro2::Span as Span2;
use std::collections::HashMap;
use syn::group::parse_braces;
use syn::group::Braces;
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
        let Braces {
            content: ref input, ..
        } = parse_braces(input)?;

        let mut fields = HashMap::new();

        loop {
            let ident: Ident = input.parse()?;
            let _: Token!(:) = input.parse()?;
            let ident_val: Ident = input.parse()?;

            fields.insert(ident, ident_val);

            if input.is_empty() {
                break;
            }

            let _: Token!(,) = input.parse()?;
        }

        Ok(Self(fields))
    }
}

impl Spanned for StructValue {
    fn span(&self) -> Span2 {
        self.0.iter().next().unwrap().0.span()
    }
}
