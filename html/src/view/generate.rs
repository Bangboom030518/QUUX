// TODO: component:0 { component:1 { element:1.1 } } element:0.1

use super::parse::Element;
use proc_macro2::TokenStream;
use quote::quote;
use shared::errors::MapInternal;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

mod client;
mod server;

static GLOBAL_ID: AtomicU64 = AtomicU64::new(0);

fn parse<T: syn::parse::Parse>(tokens: TokenStream) -> T {
    let tokens = tokens.into();
    syn::parse(tokens).expect_internal("parse tokens")
}

pub fn generate(tree: &Element) -> TokenStream {
    let server = server::generate(tree);
    GLOBAL_ID.swap(0, Relaxed);
    let client = client::generate(tree);
    quote! {
        shared::cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                {#client}
            } else {
                {#server}
            }
        }
    }
}
