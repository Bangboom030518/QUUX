use super::internal::prelude::*;
use crate::view::parse::prelude::*;

mod attributes;
mod component;
mod element;
mod for_loop;

#[derive(Clone, Default)]
pub struct Html(pub TokenStream);

impl ToTokens for Html {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.0.clone());
    }
}

impl From<Item> for Html {
    fn from(item: Item) -> Self {
        match item {
            Item::Element(element) => element.into(),
            Item::Component(component) => component.into(),
            Item::Expression(expression) => expression.into(),
        }
    }
}

fn for_loop_id(id: u64) -> Expr {
    parse_quote! {
        format!("{}.{}.{}", context.id, #id, index)
    }
}

impl Item {
    fn insert_for_loop_id(&mut self, id: u64) {
        let value = for_loop_id(id);
        let unique = match self {
            Self::Element(element) => element
                .insert_attribute("data-quux-for-id", value)
                .is_none(),
            Self::Component(component) => component.insert_for_loop_id(id).is_none(),
            Self::Expression(_) => {
                panic!("Reactive for loops must contain either elements or components. Found expression")
            }
        };
        assert!(unique, "duplicate \"data-quux-for-id\" attribute");
    }
}

impl From<Expr> for Html {
    fn from(expression: Expr) -> Self {
        Self(quote! {
            #expression.to_string()
        })
    }
}

pub fn generate(tree: &View) -> TokenStream {
    let View {
        context,
        component_enum,
        mut element,
    } = tree.clone();
    element.attributes.is_root = true;
    let Html(html) = Html::from(element.clone());

    let tokens = quote! {
        type ComponentEnum = #component_enum;

        let context = #context;
        let id = context.id;
        let mut component_id = context.id;
        let mut for_loop_children: Vec<Vec<quux::render::ClientComponentNode<ComponentEnum>>> = Vec::new();
        let mut components = Vec::<quux::render::ClientComponentNode<ComponentEnum>>::new();
        let for_loop_id = context.for_loop_id;

        quux::render::Output::new(&#html, quux::render::ClientComponentNode {
            component: ComponentEnum::from(self.clone()),
            render_context: quux::render::Context {
                id,
                children: components,
                for_loop_id: None,
                for_loop_children,
            }
        })
    };
    if element.attributes.attributes.contains_key("magic") {
        std::fs::write(
            "expansion-server.rs",
            quote! {fn main() {#tokens}}.to_string(),
        )
        .unwrap();
    }
    tokens
}
