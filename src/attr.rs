use std::collections::HashMap;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result as ParseResult;
use syn::punctuated::Punctuated;
use syn::Ident;
use syn::Token;

#[derive(Debug, Eq, PartialEq)]
pub struct MetaforAttr {
    pub replacements: HashMap<Ident, Vec<Ident>>,
}

impl Parse for MetaforAttr {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut replacements = HashMap::new();
        loop {
            // Parse an ident and create a new 'templatized' version.
            let temp_ident: Ident = input.parse()?;
            let temp_ident = Ident::new(&format!("__{}__", temp_ident), temp_ident.span());

            // Consume the equal sign.
            let _: Token!(=) = input.parse()?;

            // Parse the ident values used to populate the template variable.
            let array_group = syn::group::parse_brackets(&input)?;
            let array_parse_buffer = &array_group.content;
            let array_content: Punctuated<Ident, Token!(,)> =
                Punctuated::parse_separated_nonempty(array_parse_buffer)?;
            let replacement_idents = array_content.into_iter().collect();

            // Save the mapping of template variable -> values
            replacements.insert(temp_ident, replacement_idents);

            // If there is no more input, break out of the loop.
            if input.is_empty() {
                break;
            }

            // Consume the comma.
            let _: Token!(,) = input.parse()?;
        }

        Ok(MetaforAttr { replacements })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use proc_macro2::Span;
    use quote::quote;

    #[test]
    fn parse_attr_test() {
        let token_stream = quote! {
            i = [u8, u16]
        };

        let actual: MetaforAttr = syn::parse2(token_stream).unwrap();
        let expected = MetaforAttr {
            replacements: {
                let mut map = HashMap::new();
                map.insert(
                    Ident::new("i", Span::call_site()),
                    vec![
                        Ident::new("u8", Span::call_site()),
                        Ident::new("u16", Span::call_site()),
                    ],
                );
                map
            },
        };

        assert_eq!(actual, expected)
    }
}
