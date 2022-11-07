use core::ops::Index;
use proc_macro2::Span as Span2;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result as ParseResult;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Ident;
use syn::Token;

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct TupleValue(Punctuated<Ident, Token!(,)>);

impl TupleValue {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn values<'a>(&'a self) -> impl Iterator<Item = &'a Ident> {
        self.0.iter()
    }
}

impl Index<usize> for TupleValue {
    type Output = Ident;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Parse for TupleValue {
    fn parse(input: ParseStream) -> ParseResult<Self> {
		let content;
		syn::parenthesized!(content in input);

        let punc = Punctuated::parse_separated_nonempty(&content)?;
        Ok(Self(punc))
    }
}

impl Spanned for TupleValue {
    fn span(&self) -> Span2 {
        self.0.span()
    }
}
