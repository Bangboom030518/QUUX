// TODO: component:0 { component:1 { element:1.1 } } element:0.1

use crate::view::parse::prelude::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

pub mod client;
pub mod server;

static FOR_LOOP_ID: AtomicU64 = AtomicU64::new(0);

fn parse<T: syn::parse::Parse>(tokens: TokenStream) -> T {
    let tokens = tokens.into();
    syn::parse(tokens).unwrap()
}

pub fn generate(tree: &View) -> TokenStream {
    FOR_LOOP_ID.swap(0, Relaxed);
    let server = server::generate(tree);
    FOR_LOOP_ID.swap(0, Relaxed);
    let client = client::generate(tree);
    // TODO: move component bindings outside!!?
    quote! {
        quux::cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                {#client}
            } else {
                {#server}
            }
        }
    }
}
