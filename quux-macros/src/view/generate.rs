// TODO: component:0 { component:1 { element:1.1 } } element:0.1
use internal::prelude::*;
pub use server::Html;

mod client;
mod server;

pub fn generate(tree: &View) -> TokenStream {
    let server::Output {
        render_output: server,
        client_context,
    } = server::generate(tree);
    let client = client::generate(tree);
    quote! {
        {
            #client_context;
            quux::cfg_if::cfg_if! {
                if #[cfg(target_arch = "wasm32")] {
                    {#client}
                } else {
                    {#server}
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct ComponentDeclaration {
    ty: Type,
    ident: Ident,
}

impl ComponentDeclaration {
    fn declaration(&self) -> syn::Stmt {
        let Self { ty, ident } = self;
        parse_quote! {
            let #ident: #ty;
        }
    }
}

impl From<Component> for ComponentDeclaration {
    fn from(Component { name, ident, .. }: Component) -> Self {
        Self {
            ty: parse_quote! {
                quux::view::SerializedComponent<#name>
            },
            ident,
        }
    }
}

#[derive(Clone, Default)]
pub struct ComponentDeclarations(Vec<ComponentDeclaration>);

impl From<Vec<ComponentDeclaration>> for ComponentDeclarations {
    fn from(value: Vec<ComponentDeclaration>) -> Self {
        Self(value)
    }
}

impl ComponentDeclarations {
    fn ty(&self) -> Type {
        let Self(declarations) = self;
        let types = declarations.into_iter().map(|declaration| declaration.ty);
        parse_quote! {
            (#(#types,)*)
        }
    }

    fn expr(&self) -> Expr {
        let Self(declarations) = self;
        let idents = declarations.iter().map(|declaration| declaration.ident);

        parse_quote! {
            (#(#idents,)*)
        }
    }

    fn declarations(&self) -> TokenStream {
        let Self(declarations) = self;
        let declarations = declarations.iter().map(ComponentDeclaration::declaration);
        quote! {
            #(#declarations;)*
        }
    }
}

mod internal {
    pub mod prelude {
        pub use super::super::{ComponentDeclaration, ComponentDeclarations, Html};
        pub use crate::view::parse::prelude::*;
        pub use proc_macro2::{Ident, TokenStream};
        pub use quote::{format_ident, quote, ToTokens};
        pub use syn::{parse_quote, Expr, Type, TypePath};
    }
}
