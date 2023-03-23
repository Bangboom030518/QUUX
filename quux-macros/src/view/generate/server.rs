use super::internal::prelude::*;
use crate::view::parse::prelude::*;
use syn::parse_quote;

mod attributes;
mod component;
mod element;
mod for_loop;

#[derive(Clone)]
pub struct Html {
    pub html: syn::Expr,
    /// The types of components for a tuple for the Children type
    pub components: Vec<Component>,
    /// The types of for loop components for a tuple for the ForLoopChildren type
    pub for_loop_components: Vec<(Ident, Vec<Component>)>,
}

impl Default for Html {
    fn default() -> Self {
        Self {
            html: parse_quote! {
                String::new()
            },
            components: Vec::new(),
            for_loop_components: Vec::new(),
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
            components: Vec::new(),
            for_loop_components: Vec::new(),
        }
    }
}

// An example output of a view on the server
// ```
// Self::Output {
//     html: unimplemented!(),
//     component_node: SerializedComponent {
//         component: self,
//         render_context: Self::ClientContext {
//             components: Self::Children(child_a, child_b, ..),
//             for_loop_components: Self::ForLoops(for_loop_a, for_loop_b, ..),
//             id,
//         },
//     },
// }
// ```
/*
struct ClientContext {
    pub id: u64,
    pub components: (A, B, ..)
}
*/

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

    let ((component_types, component_idents), component_declarations): ((Vec<_>, Vec<_>), Vec<_>) =
        components
            .iter()
            .map(|Component { name, ident, .. }| {
                (
                    (name, ident),
                    quote! {
                        let #ident: quux::view::SerializedComponent<#name>;
                    },
                )
            })
            .unzip();

    let ((for_loop_types, for_loop_idents), for_loop_declarations): (
        (Vec<Type>, Vec<Ident>),
        Vec<TokenStream>,
    ) = for_loop_components
        .into_iter()
        .map(|(ident, components)| {
            // The types of components in the inner view of a for loop
            let types = components.into_iter().map(|Component { name, .. }| name);
            let ty: Type = parse_quote! {
                Vec<(#(quux::view::SerializedComponent<#types>,)*)>
            };
            (
                (ty.clone(), ident.clone()),
                quote! {
                    let #ident: #ty;
                },
            )
        })
        .unzip();

    let render_output = quote! {
        let context = #context;
        let id = context.id;
        let mut component_id = context.id;
        let for_loop_id = context.for_loop_id;

        #(#component_declarations);*
        #(#for_loop_declarations);*

        quux::view::Output::new(&#html, quux::view::SerializedComponent {
            component: self,
            render_context: ClientContext {
                id,
                for_loop_id: None,
                components: (#(#component_idents,)*),
                for_loop_components: (#(#for_loop_idents,)*),
            }
        })
    };
    // TODO: move from server
    let client_context = quote! {
        impl quux::view::ClientContext for Component {
            type Context = ClientContext;
        }

        #[derive(quux::serde::Serialize, quux::serde::Deserialize)]
        pub struct ClientContext {
            id: u64,
            for_loop_id: Option<String>,
            components: (#(quux::view::SerializedComponent<#component_types>,)*),
            for_loop_components: (#(#for_loop_types,)*),
        }
    };

    if element.attributes.attributes.contains_key("magic") {
        std::fs::write(
            "expansion-server.rs",
            quote! {fn main() {#render_output} fn context_impl() {#client_context}}.to_string(),
        )
        .unwrap();
    }
    Output {
        client_context,
        render_output,
    }
}
