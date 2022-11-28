mod client;
mod server;

#[cfg(not(target = "wasm"))]
pub use server::init_app;

#[cfg(target = "wasm")]
pub use client::init_app;
