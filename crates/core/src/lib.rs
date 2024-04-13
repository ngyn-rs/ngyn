#![doc = include_str!("../README.md")]
#[doc(hidden)]
pub mod app;
#[doc(hidden)]
pub mod platforms;

#[doc(hidden)]
pub mod macros {
    pub use async_std::main;
    pub use ngyn_macros::*;
    pub use nject::injectable as dependency;
}

#[doc(hidden)]
pub mod prelude {
    pub use crate::app::*;
    pub use crate::macros::*;
    pub use ngyn_shared::*;
}
