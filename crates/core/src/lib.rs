#![doc = include_str!("../README.md")]

pub mod macros {
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
        core::{
            engine::{NgynEngine, NgynHttpEngine, RouteInstance},
            handler::*,
        },
        server::{
            Body, JsonResponse, JsonResult, NgynContext, NgynRequest, NgynResponse, Param, Query,
            ToBytes, Transducer,
        },
        NgynGate, NgynMiddleware,
    };
}

pub mod http {
    pub use http::*;
}
