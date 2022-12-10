use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    token::{Brace, Paren},
    Expr, Ident, LitInt, Token,
};

mod test;

fn parse_html_ident(input: ParseStream) -> syn::Result<String> {
    let mut result = input.parse::<Ident>()?.to_string();

    if input.peek(Ident) {
        return Err(input.error("unexpected whitespace in html identifier"));
    }

    while !input.is_empty() {
        if input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
            result += "-";
            continue;
        }

        if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            result += ":";
            continue;
        }

        if input.peek(Token![.]) {
            input.parse::<Token![.]>()?;
            result += ".";
            continue;
        }

        if input.peek(LitInt) {
            result += &input.parse::<LitInt>()?.to_string();
            continue;
        }

        if input.peek(Ident) && !input.peek2(Ident) {
            result += &input.parse::<Ident>()?.to_string();
            continue;
        }
        break;
    }
    Ok(result)
}

#[derive(Clone)]
pub enum Item {
    Component(Component),
    Element(Element),
    Expression(Expr),
}

impl Parse for Item {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Brace) {
            let content;
            braced!(content in input);
            return Ok(Self::Expression(content.parse()?));
        }

        if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            return Ok(Self::Component(input.parse()?));
        }

        if input.peek(Ident) {
            return Ok(Self::Element(input.parse()?));
        }

        Err(input.error("Invalid Token :("))
    }
}

#[derive(Clone)]
pub struct Component {
    pub name: Ident,
    pub props: Vec<Prop>,
}

impl Parse for Component {
    // TODO: refactor
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;

        let mut props = Vec::new();
        if input.peek(Paren) {
            let attributes_buffer;
            parenthesized!(attributes_buffer in input);
            while !attributes_buffer.is_empty() {
                props.push(attributes_buffer.parse()?);
                if attributes_buffer.peek(Token![,]) {
                    attributes_buffer.parse::<Token![,]>()?;
                } else if !attributes_buffer.is_empty() {
                    return Err(
                        attributes_buffer.error("Attributes should be seperated by commas, duh!")
                    );
                }
            }
        }

        Ok(Self { name, props })
    }
}

#[derive(Clone)]
pub struct Element {
    pub tag_name: String,
    pub attributes: Vec<Attribute>,
    pub children: Children,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tag_name = parse_html_ident(input)?;

        let mut attributes = Vec::new();
        if input.peek(Paren) {
            let attributes_buffer;
            parenthesized!(attributes_buffer in input);
            while !attributes_buffer.is_empty() {
                attributes.push(attributes_buffer.parse()?);
                if attributes_buffer.peek(Token![,]) {
                    attributes_buffer.parse::<Token![,]>()?;
                } else if !attributes_buffer.is_empty() {
                    return Err(
                        attributes_buffer.error("Attributes should be seperated by commas, duh!")
                    );
                }
            }
        }

        let children;
        braced!(children in input);
        Ok(Self {
            tag_name,
            attributes,
            children: children.parse()?,
        })
    }
}

#[derive(Clone)]
pub enum Children {
    Children(Vec<Item>),
    ReactiveStore(Box<Expr>),
}

impl Parse for Children {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![$]) {
            input.parse::<Token![$]>()?;
            Ok(Self::ReactiveStore(input.parse()?))
        } else {
            let mut items = Vec::new();
            while !input.is_empty() {
                items.push(input.parse()?);
            }
            Ok(Self::Children(items))
        }
    }
}

impl Default for Children {
    fn default() -> Self {
        Self::Children(Vec::new())
    }
}

#[derive(Clone)]
pub struct Attribute {
    pub key: String,
    pub value: AttributeValue,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = parse_html_ident(input)?;
        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        Ok(Self { key, value })
    }
}

#[derive(Clone)]
pub struct Prop {
    pub key: Ident,
    pub value: Expr,
}

impl Into<TokenStream> for Prop {
    fn into(self) -> TokenStream {
        let Self { key, value } = self;
        quote! { #key: #value }
    }
}

impl Parse for Prop {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        Ok(Self { key, value })
    }
}

#[derive(Clone)]
pub enum AttributeValue {
    Reactive(Expr),
    Static(Expr),
}

impl Parse for AttributeValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![$]) {
            input.parse::<Token![$]>()?;
            Ok(Self::Reactive(input.parse()?))
        } else {
            Ok(Self::Static(input.parse()?))
        }
    }
}

