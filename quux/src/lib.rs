#![warn(clippy::pedantic, clippy::nursery)]

pub use cfg_if;
use internal::prelude::*;
pub use postcard;

pub mod component;
pub mod errors;
pub mod initialisation_script;
pub mod render;
pub mod store;

#[cfg(target_arch = "wasm32")]
pub mod dom;

#[server]
lazy_static::lazy_static! {
    pub static ref TREE_INTERPOLATION_ID: uuid::Uuid = uuid::Uuid::new_v4();
}

pub trait SerializePostcard: Serialize {
    fn serialize_bytes(&self) -> Vec<u8> {
        postcard::to_stdvec(self).expect_internal("serialize struct")
    }

    fn serialize_base64(&self) -> String {
        let bytes = self.serialize_bytes();
        base64::encode(bytes)
    }
}

mod internal {
    pub mod prelude {
        pub use super::super::{
            errors::{self, MapInternal},
            prelude::*,
            SerializePostcard,
        };
        pub use std::{
            cell::{Ref, RefCell},
            fmt::{self, Debug, Display},
            marker::PhantomData,
            rc::Rc,
            str::FromStr,
        };
    }
}

pub mod prelude {
    #[cfg(target_arch = "wasm32")]
    pub use super::dom::console_log;
    pub use super::{
        component::{self, Component, Enum as _},
        initialisation_script::InitialisationScript,
        render,
        store::{self, Store},
    };
    pub use quux_macros::{client, init_components, server, view};
    pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
}
