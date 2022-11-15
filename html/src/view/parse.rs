use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream},
    token::{Brace, Paren},
    Expr, Ident, Token,
};
use syn_serde::json;

#[derive(Clone)]
pub enum Item {
    ReactiveStore(Ident),
    Component(Component),
    Element(Element),
    Expression(Expr),
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReactiveStore(ident) => write!(f, "Ident \"{}\"", ident),
            Self::Element(elem) => write!(f, "{}", elem),
            Self::Expression(expr) => write!(f, "{}", json::to_string_pretty(expr)),
            Self::Component(coponent) => write!(f, "not for now"),
        }
    }
}

impl Parse for Item {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Brace) {
            let content;
            braced!(content in input);
            return Ok(Self::Expression(content.parse()?));
        }

        if input.peek(Token![$]) && input.peek2(Ident) {
            input.parse::<Token![$]>()?;
            return Ok(Self::ReactiveStore(input.parse()?));
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
    pub props: Vec<Attribute>,
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

        Ok(Self {
            name,
            props,
        })
    }
}

#[derive(Clone)]
pub struct Element {
    pub tag_name: Ident,
    pub attributes: Vec<Attribute>,
    pub content: Vec<Item>,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tag_name = input.parse()?;

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

        let content_buffer;
        braced!(content_buffer in input);
        let mut content = Vec::new();
        while !content_buffer.is_empty() {
            content.push(content_buffer.parse()?);
        }

        Ok(Self {
            tag_name,
            attributes,
            content,
        })
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Element {{\n tag_name: {},\n attributes: {:?},\n content: {:?} \n}}",
            &json::to_string_pretty(&self.tag_name),
            &self
                .attributes
                .iter()
                .map(|attribute| attribute.to_string())
                .collect::<Vec<_>>(),
            &self
                .content
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<_>>()
        )?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct Attribute {
    pub key: Ident,
    pub value: AttributeValue,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        Ok(Self { key, value })
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{\n key: \"{}\",\n value: \"{}\" \n}}",
            self.key, self.value
        )
    }
}

#[derive(Clone)]
pub enum AttributeValue {
    Reactive(Ident),
    Static(Box<Expr>),
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

impl std::fmt::Display for AttributeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reactive(ident) => write!(f, "Ident \"{}\"", ident),
            Self::Static(expr) => write!(f, "{}", json::to_string_pretty::<Expr>(expr)),
        }
    }
}
