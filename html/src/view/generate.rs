mod server;
mod client;

#[cfg(not(target_arch="wasm32"))]
pub use server::generate;

#[cfg(target_arch="wasm32")]
pub use client::generate;