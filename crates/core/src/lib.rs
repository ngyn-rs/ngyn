#![doc = include_str!("../README.md")]
pub mod app;

pub mod macros {
    pub use async_std::main;
    pub use ngyn_macros::*;
}

#[doc(hidden)]
pub mod prelude {
    pub use crate::app::*;
    pub use crate::macros::*;
    pub use ngyn_shared::*;
}

pub mod http {
    pub use hyper::http::*;
}
