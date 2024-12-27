use std::sync::Arc;

pub type ContextProviders = Arc<Vec<Box<(dyn Fn() -> Box<(dyn std::any::Any)> + Send + Sync)>>>;

pub mod components;
#[cfg(feature = "server")]
pub mod db;
mod models;
#[cfg(feature = "server")]
mod schema;
#[cfg(feature = "server")]
pub mod server;
mod server_functions;
