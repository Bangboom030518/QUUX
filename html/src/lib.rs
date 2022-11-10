use proc_macro::TokenStream;
use quote::quote;
use std::fs;
use syn::{
    braced,
    parse::{Parse, ParseBuffer, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::{self, Brace},
    Expr, Field, Ident, Token,
};

const LOG_PATH: &str = "./log";

#[derive(Clone)]
enum Item {
    ReactiveStore(Ident),
    Element(Element),
    Expression(Expr),
}

impl Parse for Item {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Brace) {
            let content;
            braced!(content in input);
            log(&content);
            return Ok(Item::Expression(content.parse().expect("Uh Oh!")));
        }

        Err(input.error("Invalid Token :("))
    }
}

#[derive(Clone)]
struct Element {
    tag_name: Ident,
    attributes: Punctuated<Attribute, Token![,]>,
    content: Vec<Item>,
}

#[derive(Clone)]
struct Attribute {
    key: Ident,
    value: AttributeValue,
}

#[derive(Clone)]
enum AttributeValue {
    Reactive(Ident),
    Static(Expr),
}

/// $ident:tag_name ( $( $ident:key = $expr:value )* ) {
///     (!($$ $ident:reactive_store) $self)* | $$ $ident:reactive_store
/// }
///
/// { $expr:content }
///
/// $$ $ident:reactive_store

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    fs::write(LOG_PATH, "").unwrap();
    let tree = parse_macro_input!(input as Item);
    quote! {
        println!("Hello!")
    }
    .into()
}

#[test]
fn test_element() {
    html(
        quote! {
            button(class="btn") {
                { "Click Me" }
            }
        }
        .into(),
    );
}

#[test]
fn test_expr() {
    html(
        quote! {
            { "always watching" }
        }
        .into(),
    );
}

#[inline]
fn log(append_str: impl std::fmt::Display) {
    let contents = fs::read_to_string(LOG_PATH).unwrap();
    fs::write(LOG_PATH, format!("{}\n{}", contents, append_str)).unwrap();
}
