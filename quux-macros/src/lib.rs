#![warn(clippy::pedantic, clippy::nursery)]
#![feature(exact_size_is_empty, iter_intersperse)]
use proc_macro::TokenStream;

mod view;

// TODO: document
#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    view::view(input)
}

