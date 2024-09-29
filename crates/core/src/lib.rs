#![doc = include_str!("../README.md")]
pub mod factory;

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
    pub use crate::factory::*;
    pub use crate::macros::*;
    pub use ngyn_hyper::HyperApplication;
    pub use ngyn_shared::{
        core::{handler::*, engine::NgynEngine},
        server::{
            Body, JsonResponse, JsonResult, NgynContext, NgynRequest, NgynResponse, Param, Query,
            Transducer,
        },
        traits::{NgynGate, NgynInjectable, NgynMiddleware},
    };
}

pub mod http {
    pub use http::*;
}
