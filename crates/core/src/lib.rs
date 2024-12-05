#![doc = include_str!("../README.md")]

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
    pub use crate::macros::*;
    pub use ngyn_hyper::HyperApplication;
    pub use ngyn_shared::{
        core::{engine::NgynEngine, handler::*},
        server::{
            Body, JsonResponse, JsonResult, NgynContext, NgynRequest, NgynResponse, Param, Query,
            ToBytes, Transducer,
        },
        traits::{NgynGate, NgynMiddleware},
    };
}

pub mod http {
    pub use http::*;
}
