#![warn(clippy::pedantic, clippy::nursery)]
#![allow(incomplete_features)]
#![feature(return_position_impl_trait_in_trait, drain_filter)]

pub use cfg_if;
use internal::prelude::*;
pub use postcard;
pub use quux_macros as macros;
pub use serde;
#[cfg(all(feature = "warp", not(target_arch = "wasm32")))]
pub use warp;

pub mod component;
pub mod context;
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
        #[client]
        pub use super::super::tree::item::DomRepresentation;
        pub use super::super::{
            errors::{self, MapInternal},
            prelude::*,
            tree::{prelude::*, Hydrate},
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
    #[client]
    pub use super::dom::console_log;
    pub use super::{
        component::{self, Component, Init as _, Routes as _},
        context::Context,
        event,
        initialisation_script::InitialisationScript,
        store::{self, Store},
        tree::{
            element::html::prelude::*,
            item::{branch::prelude::*, children, Many},
            Item,
        },
    };

    #[cfg(feature = "warp")]
    #[macro_export]
    macro_rules! routes {
        ($($tokens:tt)*) => {
            quux::macros::routes!(#warp $($tokens)*);
        };
    }
    #[cfg(feature = "warp")]
    pub use routes;

    #[cfg(not(feature = "warp"))]
    pub use quux_macros::routes;

    pub use quux_macros::{client, server, view};
    pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
}
