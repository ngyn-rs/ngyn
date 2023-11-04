pub mod app;
pub mod platforms;

pub use crate::app::factory::*;
pub use crate::app::provider::*;
pub use async_std::main;
pub use ngyn_macros::*;
pub use ngyn_shared::*;
pub use nject::{injectable as dependency, provider};

#[cfg(feature = "tide")]
pub type Result<T> = tide::Result<T>;
