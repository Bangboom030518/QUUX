use super::parse::Element;
use proc_macro2::TokenStream;
use quote::quote;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

mod client;
mod server;

pub static GLOBAL_ID: AtomicU64 = AtomicU64::new(0);

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
