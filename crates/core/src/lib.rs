#![doc = include_str!("../README.md")]
pub mod app;

pub mod macros {
    pub use async_std::main;
    pub use async_trait::async_trait;
    pub use ngyn_macros::*;
}

pub mod shared {
    pub use ngyn_shared::*;
}

#[doc(hidden)]
pub mod prelude {
    pub use crate::app::*;
    pub use crate::macros::*;
    pub use ngyn_shared::{
        core::NgynEngine,
        server::{
            Body, FullResponse, NgynContext, NgynRequest, NgynResponse, Param, Query, Transducer,
            Transformer,
        },
        traits::{NgynControllerHandler, NgynGate, NgynInjectable, NgynMiddleware},
    };
}

pub mod http {
    pub use hyper::http::*;
}
