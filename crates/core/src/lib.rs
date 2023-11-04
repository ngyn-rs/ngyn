pub mod app;
pub mod server;

pub use crate::app::factory::*;
pub use crate::app::provider::*;
pub use async_std::main;
pub use ngyn_macros::*;
pub use ngyn_shared::*;
pub use nject::{injectable as dependency, provider};

#[cfg(feature = "core")]
pub type Result<T> = tide::Result<T>;
