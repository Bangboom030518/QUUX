use super::internal::prelude::*;
use crate::view::parse::prelude::*;
use syn::parse_quote;

mod attributes;
mod component;
mod element;
mod for_loop;
mod if_expr;
mod match_expr;

#[derive(Clone)]
pub struct Html {
    pub html: syn::Expr,
    /// The types of components for a tuple for the Children type
    pub components: Components,
    /// The types of for loop components for a tuple for the ForLoopChildren type
    pub for_loop_components: ForLoops,
}

impl Default for Html {
    fn default() -> Self {
        Self {
            html: parse_quote! {
                String::new()
            },
            components: Components::default(),
            for_loop_components: ForLoops::default(),
        }
    }
}

impl ToTokens for Html {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.html.to_tokens(tokens);
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
        Self {
            html: parse_quote! {
                #expression.to_string()
            },
            components: Components::default(),
            for_loop_components: ForLoops::default(),
        }
    }
}

pub struct Output {
    pub client_context: TokenStream,
    pub render_output: TokenStream,
}

pub fn generate(tree: &View) -> Output {
    let View {
        context,
        mut element,
    } = tree.clone();
    element.attributes.is_root = true;
    let Html {
        html,
        components,
        for_loop_components,
    } = Html::from(element.clone());

    let components_type = components.ty();
    let components_expr = components.expr();
    let components_declarations = components.declarations();
    let for_loops_type = for_loop_components.ty();
    let for_loops_expr = for_loop_components.expr();
    let for_loops_declarations = for_loop_components.declarations();

    let render_output = quote! {
        use quux::{view::{Output, ClientContext, ServerContext, SerializedComponent}, component::Component as _};
        let context = #context;
        let id = context.id;
        let mut component_id = context.id;
        let for_loop_id = context.for_loop_id;

        #components_declarations
        #for_loops_declarations

        Output::new(&#html, SerializedComponent::new(self, ClientContext::new(id, None, #components_expr, #for_loops_expr)))
    };
    // TODO: move from server
    let client_context = quote! {
        impl quux::view::ComponentChildren for Component {
            type Components = #components_type;
            type ForLoopComponents = #for_loops_type;
        }
    };

    if element.attributes.attributes.contains_key("magic") {
        std::fs::write(
            "expansion-server.rs",
            quote! {
                fn main() {
                    #render_output
                }
                fn context_impl() {
                    #client_context
                }
            }
            .to_string(),
        )
        .unwrap();
    }
    Output {
        client_context,
        render_output,
    }
}
