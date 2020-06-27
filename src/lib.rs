mod attr;

use self::attr::MetaforAttr;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use std::collections::HashMap;
use syn::parse_macro_input;
use syn::visit_mut::visit_ident_mut;
use syn::visit_mut::VisitMut;
use syn::Ident;
use syn::Item;

struct IdentReplacer<'a> {
    mapping: HashMap<&'a Ident, &'a Ident>,
}

impl<'a> VisitMut for IdentReplacer<'a> {
    fn visit_ident_mut(&mut self, ident: &mut Ident) {
        if let Some(new_ident) = self.mapping.get(ident) {
            *ident = (*new_ident).clone();
        }

        visit_ident_mut(self, ident);
    }
}

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
            replace_idents(&mut dup_input, mapping);
            dup_input
        })
        .collect()
}

fn generate_mappings<'a>(
    attr: &'a MetaforAttr,
) -> impl Iterator<Item = HashMap<&'a Ident, &'a Ident>> {
    let replacements = &attr.replacements;

    // Creates a counter for each template identifier.
    let mut counters = HashMap::new();
    for (template_ident, _) in replacements.iter() {
        counters.insert(template_ident, 0usize);
    }

    let mut has_next_combo = true;
    core::iter::from_fn::<HashMap<&'a Ident, &'a Ident>, _>(move || {
        if !has_next_combo {
            return None;
        }

        let mut mapping: HashMap<&'a Ident, &'a Ident> = HashMap::new();

        // Populate the map to return
        for (template_ident, counter) in counters.iter() {
            let replacement_ident = &replacements[*template_ident][*counter];
            mapping.insert(*template_ident, replacement_ident);
        }

        // Iterate the next combination
        has_next_combo = false;
        for (template_ident, counter) in counters.iter_mut() {
            let max_count = replacements[template_ident].len();
            *counter = *counter + 1;

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

fn replace_idents(input: &mut Item, mapping: HashMap<&Ident, &Ident>) {
    let mut ident_replacer = IdentReplacer { mapping };

    match input {
        Item::Impl(impl_item) => ident_replacer.visit_item_impl_mut(impl_item),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use proc_macro2::Span as Span2;
    use syn::parse_quote;

    #[test]
    fn impl_metafor_test() {
        let input = parse_quote! {
            impl MyTrait for __i__ {}
        };

        let i_ident = Ident::new("__i__", Span2::call_site());
        let u8_ident = Ident::new("u8", Span2::call_site());
        let u16_ident = Ident::new("u16", Span2::call_site());

        let attr = MetaforAttr {
            replacements: {
                let mut map = HashMap::new();
                map.insert(i_ident, vec![u8_ident, u16_ident]);
                map
            },
        };

        let actual = impl_metafor(input, attr);
        let expected = vec![
            parse_quote!(impl MyTrait for u8 {}),
            parse_quote!(impl MyTrait for u16 {}),
        ];

        assert_eq!(actual, expected)
    }

    #[test]
    fn replace_idents_test() {
        let mut input: Item = parse_quote! {
            impl MyTrait for __i__ {}
        };

        let temp_ident = Ident::new("__i__", Span2::call_site());
        let temp_value = Ident::new("u8", Span2::call_site());
        replace_idents(&mut input, {
            let mut map = HashMap::new();
            map.insert(&temp_ident, &temp_value);
            map
        });

        let expected: Item = parse_quote! {
            impl MyTrait for u8 {}
        };

        assert_eq!(input, expected)
    }
}
