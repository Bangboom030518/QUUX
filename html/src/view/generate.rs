mod server;
mod client;

#[cfg(not(target="wasm"))]
pub use server::generate;

#[cfg(target="wasm")]
pub use client::generate;