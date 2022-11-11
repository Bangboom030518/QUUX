use quote::quote;
use proc_macro::TokenStream;
use super::Item;

pub fn generate(tree: Item) -> TokenStream {
    quote! {
        todo!("Implement this macro")
    }.into()
}
