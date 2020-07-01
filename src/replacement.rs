use std::collections::HashMap;
use syn::visit_mut::visit_ident_mut;
use syn::visit_mut::VisitMut;
use syn::Ident;

pub struct IdentReplacer<'a> {
    mapping: HashMap<Ident, &'a Ident>,
}

impl<'a> IdentReplacer<'a> {
    pub fn new(mapping: HashMap<Ident, &'a Ident>) -> Self {
        Self { mapping }
    }
}

impl<'a> VisitMut for IdentReplacer<'a> {
    fn visit_ident_mut(&mut self, ident: &mut Ident) {
        if let Some(new_ident) = self.mapping.get(ident) {
            *ident = (*new_ident).clone();
        }

        visit_ident_mut(self, ident);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use proc_macro2::Span as Span2;
    use syn::parse_quote;
    use syn::Ident;
    use syn::Item;

    #[test]
    fn replacement_test() {
        // Define the idents that will be used in the mapping.
        let temp_ident = Ident::new("__i__", Span2::call_site());
        let temp_value = Ident::new("u8", Span2::call_site());

        // Define the actual result and the expected result
        let mut actual: Item = parse_quote!(impl MyTrait for __i__ {});
        let expected: Item = parse_quote!(impl MyTrait for u8 {});

        // Remap the idents in the input
        IdentReplacer::new({
            let mut map = HashMap::new();
            map.insert(temp_ident, &temp_value);
            map
        })
        .visit_item_mut(&mut actual);

        assert_eq!(actual, expected)
    }
}
