#![warn(clippy::pedantic, clippy::nursery)]
#![feature(exact_size_is_empty, iter_intersperse)]
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::parse_macro_input;

mod view;

// TODO: document
// TODO: accept context and component enum type in a parameter-like manner
#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    let tree = parse_macro_input!(input as view::Element);
    view::generate(&tree).into()
}

fn parse<T: syn::parse::Parse>(tokens: TokenStream2) -> T {
    let tokens = tokens.into();
    syn::parse(tokens).unwrap()
}
