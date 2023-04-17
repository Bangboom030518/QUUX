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
    pub ty: syn::Type,
}

impl Html {
    pub fn new(html: syn::Expr, ty: syn::Type) -> Self {
        Self { html, ty }
    }
}

impl Default for Html {
    fn default() -> Self {
        Self {
            html: parse_quote! {
                String::new()
            },
            ty: parse_quote! {
                ()
            },
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
            ty: parse_quote! {
                String
            },
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
    let Html { html, ty } = Html::from(element);

    let render_output = quote! {
        use quux::{view::{output, ClientContext, ServerContext, SerializedComponent}, component::Component as _};
        let context = #context;
        let id = context.id;
        let mut component_id = context.id;
        let for_loop_id = context.for_loop_id;

       output::Server::new(&#html, SerializedComponent::new(__self, ClientContext::new(id, None, #components_expr, #for_loops_expr)))
    };
    // TODO: move from server
    let client_context = quote! {
        let __self = self.clone();

        #[cfg(target_arch = "wasm32")]
        let render_server = {
            use quux::view::ServerContext;
            let #context = ServerContext::<Self>::new(#context.id, #context.for_loop_id.clone());
            let __self = __self.clone();
            move || {{
                #render_output
            }}
        };

        impl quux::view::ComponentChildren for Component {
            type Children = #ty;
        }
    };
    Output {
        client_context,
        render_output: quote! {
            render_server()
        },
    }
}
