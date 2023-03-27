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

pub trait AsDeclaration {
    fn ty(&self) -> Type;
    fn ident(&self) -> Ident;

    fn declaration(&self) -> syn::Stmt {
        let ty = self.ty();
        let ident = self.ident();

        parse_quote! {
            let #ident: #ty;
        }
    }
}

impl AsDeclaration for ForLoop {
    fn ty(&self) -> Type {
        let item = self.item.clone();
        let Html { components, .. } = (*item).into();

        let ty = components.ty();
        parse_quote! { Vec<#ty> }
    }
    fn ident(&self) -> Ident {
        let id = self.id;
        format_ident!("for_loop_components_{id}")
    }
}

impl AsDeclaration for Component {
    fn ty(&self) -> Type {
        let ty = &self.name;
        parse_quote! {
            quux::view::SerializedComponent<#ty>
        }
    }

    fn ident(&self) -> Ident {
        self.ident.clone()
    }
}
#[derive(Clone, Default)]
pub struct ForLoops(Vec<ForLoop>);

impl AsDeclarations for ForLoops {
    type Item = ForLoop;

    fn items(&self) -> &[Self::Item] {
        &self.0
    }
}

#[derive(Clone, Default)]
pub struct Components(Vec<Component>);

impl AsDeclarations for Components {
    type Item = Component;

    fn items(&self) -> &[Self::Item] {
        &self.0
    }
}

impl Components {
    pub fn bindings(&self) -> Vec<&Ident> {
        self.0
            .iter()
            .filter_map(|component| component.binding.as_ref())
            .collect()
    }

    pub fn types(&self) -> Vec<&syn::Path> {
        self.0.iter().map(|component| &component.name).collect()
    }
}

pub trait AsDeclarations {
    type Item: AsDeclaration;

    fn items(&self) -> &[Self::Item];

    fn ty(&self) -> Type {
        let items = self.items();
        let types = items.iter().map(AsDeclaration::ty);
        parse_quote! {
            (#(#types,)*)
        }
    }

    fn expr(&self) -> Expr {
        let items = self.items();
        let idents = items.iter().map(AsDeclaration::ident);

        parse_quote! {
            (#(#idents,)*)
        }
    }

    fn declarations(&self) -> TokenStream {
        let items = self.items();
        let declarations = items.iter().map(AsDeclaration::declaration);
        quote! {
            #(#declarations;)*
        }
    }
}

mod internal {
    pub mod prelude {
        pub use super::super::{AsDeclaration, AsDeclarations, Components, ForLoops, Html};
        pub use crate::view::parse::prelude::*;
        pub use proc_macro2::{Ident, TokenStream};
        pub use quote::{format_ident, quote, ToTokens};
        pub use syn::{parse_quote, Expr, Type, TypePath};
    }
}
