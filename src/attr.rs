mod structs;
mod tuples;

use self::structs::StructValue;
use self::tuples::TupleValue;
use std::collections::HashMap;
use syn::parse::Error as ParseError;
use syn::parse::Parse;
use syn::parse::ParseBuffer;
use syn::parse::ParseStream;
use syn::parse::Result as ParseResult;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::Ident;
use syn::Token;

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub struct MetaforAttr {
    pub replacements: HashMap<Ident, TemplateValues>,
}

impl Parse for MetaforAttr {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut replacements = HashMap::new();
        loop {
            // Parse an ident that acts as the template parameter
            let temp_ident: Ident = input.parse()?;

            // Consume the equal sign.
            let _: Token!(=) = input.parse()?;

            // Parse the ident values used to populate the template variable.
            let template_values = input.parse()?;

            // Save the mapping of template variable -> values
            replacements.insert(temp_ident, template_values);

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

#[cfg_attr(test, derive(Debug, Eq, PartialEq))]
pub enum TemplateValues {
    Idents(Punctuated<Ident, Token!(,)>),
    Tuples(Punctuated<TupleValue, Token!(,)>),
    Structs(Punctuated<StructValue, Token!(,)>),
}

impl TemplateValues {
    pub fn len(&self) -> usize {
        match self {
            TemplateValues::Idents(idents) => idents.len(),
            TemplateValues::Tuples(tuples) => tuples.len(),
            TemplateValues::Structs(structs) => structs.len(),
        }
    }

    fn validate(self) -> ParseResult<Self> {
        match &self {
            TemplateValues::Idents(_) => Ok(self),
            TemplateValues::Tuples(tuples) => {
                let count = tuples.first().unwrap().len();
                if let Some(tuple) = tuples.iter().find(|tuple| tuple.len() != count) {
                    let span = tuple.span();
                    let message = format!("Expected {} elements but found {}", count, tuple.len());
                    return Err(ParseError::new(span, message));
                }

                return Ok(self);
            }
            TemplateValues::Structs(structs) => {
                fn get_struct_fields(struct_val: &StructValue) -> HashMap<String, ()> {
                    struct_val
                        .fields()
                        .map(|(field, _)| (field.to_string(), ()))
                        .collect()
                }

                let field_names = get_struct_fields(structs.first().unwrap());

                let offending_struct = structs
                    .iter()
                    .find(|struct_val| !get_struct_fields(struct_val).eq(&field_names));

                if let Some(offending_struct) = offending_struct {
                    let span = offending_struct.span();
                    let message = format!(
                        "Expected struct to have the fields [{}]",
                        field_names
                            .keys()
                            .into_iter()
                            .map(ToOwned::to_owned)
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                    return Err(ParseError::new(span, &message));
                }

                return Ok(self);
            }
        }
    }
}

impl Parse for TemplateValues {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let content: ParseBuffer;
        syn::bracketed!(content in input);

        fn parse_idents(input: ParseStream) -> ParseResult<TemplateValues> {
            Punctuated::parse_separated_nonempty(input).map(|punc| TemplateValues::Idents(punc))
        }

        fn parse_tuples(input: ParseStream) -> ParseResult<TemplateValues> {
            Punctuated::parse_separated_nonempty(input).map(|punc| TemplateValues::Tuples(punc))
        }

        fn parse_structs(input: ParseStream) -> ParseResult<TemplateValues> {
            Punctuated::parse_separated_nonempty(input).map(|punc| TemplateValues::Structs(punc))
        }

        let template_values = parse_idents(&content)
            .or_else(|_| parse_tuples(&content))
            .or_else(|_| parse_structs(&content))?;

        template_values.validate()
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
                    TemplateValues::Idents(syn::parse_quote!(u8, u16)),
                );
                map
            },
        };

        assert_eq!(actual, expected)
    }
}
