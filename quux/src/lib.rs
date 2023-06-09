#![warn(clippy::pedantic, clippy::nursery)]
#![allow(incomplete_features)]
#![feature(return_position_impl_trait_in_trait, drain_filter)]

pub use cfg_if;
use internal::prelude::*;
pub use postcard;
pub use quux_macros as macros;
#[cfg(not(target_arch = "wasm32"))]
pub use quux_server as server;
pub use serde;

pub mod component;
pub mod errors;
pub mod initialisation_script;
pub mod store;
pub mod tree;

#[cfg(target_arch = "wasm32")]
pub mod dom;

pub trait SerializePostcard: Serialize {
    fn serialize_bytes(&self) -> Vec<u8> {
        postcard::to_stdvec(self).expect_internal("serialize struct")
    }

    fn serialize_base64(&self) -> String {
        let bytes = self.serialize_bytes();
        base64::encode(bytes)
    }

    /// # Errors
    /// - If the string is unparseable
    fn deserialize_base64(string: &str) -> Result<Self, errors::ClientParse>
    where
        Self: DeserializeOwned + Sized,
    {
        let bytes = base64::decode(string).map_err(errors::ClientParse::Base64Decode)?;
        let node = postcard::from_bytes(&bytes).map_err(errors::ClientParse::PostcardDecode)?;
        Ok(node)
    }
}

mod internal {
    pub mod prelude {
        #[cfg_client]
        pub use super::super::tree::item::DomRepresentation;
        pub use super::super::{
            errors::{self, MapInternal},
            prelude::*,
            tree::prelude::*,
            SerializePostcard,
        };
        pub use std::{
            cell::{Ref, RefCell},
            collections::HashMap,
            fmt::{self, Debug, Display},
            marker::PhantomData,
            rc::Rc,
            str::FromStr,
        };
    }
}

pub mod prelude {
    #[cfg_server]
    pub use super::component::ServerExt as _;
    #[cfg_client]
    pub use super::dom::console_log;
    pub use super::{
        component::{self, Component, Routes as _},
        event,
        initialisation_script::InitialisationScript,
        store::{self, Store},
        tree::{
            element::html::prelude::*,
            item::{branch::prelude::*, children, Many},
            Item,
        },
    };
    pub use quux_macros::{client as cfg_client, routes, server as cfg_server, view};
    #[cfg_server]
    pub use quux_server::{self, prelude::*};
    pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
}
