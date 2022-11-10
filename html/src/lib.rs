use derive_syn_parse::Parse;
use proc_macro::TokenStream;
use quote::quote;
use std::fs;
use syn::{
    braced,
    parse::{Parse, ParseStream, ParseBuffer},
    parse_macro_input,
    punctuated::Punctuated,
    token::{self},
    Expr, Field, Ident, Token,
};

const LOG_PATH: &str = "./log";

fn test_element(token: &ParseBuffer) -> bool {
    log(&token.to_string());
    true
}

#[derive(Clone, Parse)]
enum Item {
    #[peek(Ident, name = "ReactiveStore")]
    ReactiveStore(Ident),
    #[peek_with(test_element, name = "Element")]
    Element(Element),
    #[peek_with(|_| true, name = "Expression")]
    Expression(Expr),
}
// impl Parse for Item {

// }

#[derive(Clone, Parse)]
struct Element {
    tag_name: Ident,
    #[paren]
    paren: token::Paren,
    // #[inside(paren)]
    #[call(syn::Attribute::parse_outer)]
    attributes: Punctuated<Attribute, Token![,]>,
    #[call(syn::Attribute::parse_outer)]
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

// impl Parse for Item {
//     fn parse(input: ParseStream) -> syn::Result<Self> {

//         let lookahead = input.lookahead1();
//         if lookahead.peek(Ident) {
//             input.parse().map(Item::)
//         } else if lookahead.peek(token::Brace) {
//             input.parse().map(Item::Enum)
//         } else  {
//             Err(lookahead.error())
//         }
//     }
// }

#[derive(Clone)]
struct ItemStruct {
    struct_token: Token![struct],
    ident: Ident,
    brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for ItemStruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(ItemStruct {
            struct_token: input.parse()?,
            ident: input.parse()?,
            brace_token: braced!(content in input),
            fields: content.parse_terminated(Field::parse_named)?,
        })
    }
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
    for token in input {
        // log(&format!("{:?}\n", token))
    }
    let tree = parse_macro_input!(input as Item);
    log(&format!("{}", quote! { tree}));
    quote! {
        println!("Hello!")
    }
    .into()
}

#[test]
fn test_html() {
    html(
        quote! {
            button(class="btn") {
                { "Click Me" }
            }
        }
        .into(),
    );
}

#[inline]
fn log(append_str: &str) {
    let contents = fs::read_to_string(LOG_PATH).unwrap();
    fs::write(LOG_PATH, contents + append_str).unwrap();
}
