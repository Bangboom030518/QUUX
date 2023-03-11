#![warn(clippy::pedantic, clippy::nursery)]
#![feature(exact_size_is_empty, iter_intersperse)]
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

mod view;

// TODO: document
#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    view::view(input)
}

fn parse<T: syn::parse::Parse>(tokens: TokenStream2) -> T {
    let tokens = tokens.into();
    syn::parse(tokens).unwrap()
}
