#![warn(clippy::all)]
#![warn(clippy::cargo_common_metadata)]

mod attr;
mod replacement;

use self::attr::MetaforAttr;
use self::attr::TemplateValues;
use self::replacement::IdentReplacer;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use std::collections::HashMap;
use syn::parse_macro_input;
use syn::visit_mut::VisitMut;
use syn::Ident;
use syn::Item;

#[proc_macro_attribute]
pub fn metafor(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    let attr = parse_macro_input!(attr as MetaforAttr);

    impl_metafor(input, attr)
        .into_iter()
        .map(|item| item.to_token_stream())
        .collect::<TokenStream2>()
        .into()
}

fn impl_metafor(input: Item, attr: MetaforAttr) -> Vec<Item> {
    generate_mappings(&attr)
        .map(|mapping| {
            let mut dup_input = input.clone();

            let mut ident_replacer = IdentReplacer::new(mapping);
            ident_replacer.visit_item_mut(&mut dup_input);

            dup_input
        })
        .collect()
}

fn generate_mappings<'a>(attr: &'a MetaforAttr) -> impl Iterator<Item = HashMap<Ident, &'a Ident>> {
    let replacements = &attr.replacements;

    // Creates a counter for each template identifier.
    let mut counters = HashMap::new();
    for (template_ident, _) in replacements.iter() {
        counters.insert(template_ident, 0usize);
    }

    let mut has_next_combo = true;
    core::iter::from_fn(move || {
        if !has_next_combo {
            return None;
        }

        let mut mapping = HashMap::new();

        // Populate the map to return
        for (temp_ident, counter) in counters.iter() {
            match &replacements[*temp_ident] {
                TemplateValues::Idents(idents) => {
                    // Get the identifier that will replace the template parameter
                    let replacement_ident = &idents[*counter];

                    // Create the new identifier to replace
                    let string = format!("__{}__", temp_ident);
                    let temp_ident = Ident::new(&string, temp_ident.span());

                    // Record the mapping
                    mapping.insert(temp_ident, replacement_ident);
                }
                TemplateValues::Tuples(tuples) => {
                    let temp_tuple = &tuples[*counter];

                    for (i, temp_tuple_value) in temp_tuple.values().enumerate() {
                        let string = format!("__{}__{}__", temp_ident, i);
                        let temp_ident = Ident::new(&string, temp_ident.span());

                        mapping.insert(temp_ident, temp_tuple_value);
                    }
                }
                TemplateValues::Structs(structs) => {
                    let temp_struct = &structs[*counter];

                    for (struct_field, temp_struct_value) in temp_struct.fields() {
                        let string = format!("__{}__{}__", temp_ident, struct_field);
                        let temp_ident = Ident::new(&string, temp_ident.span());

                        mapping.insert(temp_ident, temp_struct_value);
                    }
                }
            };
        }

        // Iterate the next combination
        has_next_combo = false;
        for (template_ident, counter) in counters.iter_mut() {
            let max_count = replacements[template_ident].len();
            *counter += 1;

            // If the current counter has reached its max, reset it and move to the next one.
            if *counter >= max_count {
                *counter = 0;
            } else {
                has_next_combo = true;
                break;
            }
        }

        Some(mapping)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use syn::parse_quote;

    #[test]
    fn impl_metafor_test() {
        let input = parse_quote! {
            impl MyTrait for __i__ {}
        };

        let attr = parse_quote! {
            i = [u8, u16]
        };

        let actual = impl_metafor(input, attr);
        let expected = vec![
            parse_quote!(impl MyTrait for u8 {}),
            parse_quote!(impl MyTrait for u16 {}),
        ];

        assert_eq!(actual, expected)
    }
}
