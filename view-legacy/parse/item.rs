use super::internal::prelude::*;

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
