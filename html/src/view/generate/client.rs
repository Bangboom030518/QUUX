#![cfg(target_arch="wasm32")]

use super::super::parse::Item;
use quote::quote;
use proc_macro2::TokenStream;

pub fn generate(tree: Item) -> TokenStream {
    quote! {
        todo!("Add WASM logic")
    }
}
