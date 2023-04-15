use super::super::internal::prelude::*;

fn hashmap_tokens<K, V>(map: Vec<(K, V)>, include_for_loop_id: bool) -> Expr
where
    K: ToTokens,
    V: ToTokens,
{
    let attributes = map.into_iter().map(|(key, value)| {
        quote! {
            (#key, #value)
        }
    });
    let for_loop_id = include_for_loop_id.then(|| {
        quote! {
            &if let Some(id) = for_loop_id {
                Some(String::from("data-quux-for-id"), id)
            } else {
                None
            }
        }
    }).unwrap_or_default();
    parse_quote! {
        std::collections::HashMap::from_iter(vec![#(Some(#attributes)),*, #for_loop_id].into_iter().flatten())
    }
}

impl Attributes {
    /// Adds the scoped id attribute with the value of `id` if the containing element needs an id because it is reactive.
    /// If the element is not reactive, nothing is added.
    fn insert_scoped_id(&mut self) {
        if !self.element_needs_id {
            return;
        }
        let id = self.id;
        self.attributes.insert(
            "data-quux-id".to_string(),
            parse_quote!(format!("{}.{}", &id, #id)),
        );
    }

    fn static_attributes(&self) -> Expr {
        let attributes = self.attributes.into_iter().map(|(key, value)| {
            (
                quote! {
                    String::from(#key)
                },
                quote! {
                    String::from(#value)
                },
            )
        });
        hashmap_tokens(attributes.collect(), self.is_root)
    }
    fn reactive_attributes(&self) -> Expr {
        let attributes = self.attributes.into_iter().map(|(key, value)| {
            (
                quote! {
                    String::from(#key)
                },
                value,
            )
        });
        hashmap_tokens(attributes.collect(), false)
    }
}

impl ToTokens for Attributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut attributes = self.clone();
        self.insert_scoped_id();
        let static_attributes = attributes.static_attributes();
        let reactive_attributes = attributes.reactive_attributes();
        quote! {
            quux::tree::Attributes::new(#static_attributes, #reactive_attributes)
        }
        .to_tokens(tokens)
    }
}
