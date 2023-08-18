pub mod traits {
    pub mod controller;
    pub mod injectable;
    pub mod module;
}

pub mod common {
    pub mod provider;
}

pub use crate::common::provider::*;
pub use crate::traits::controller::*;
pub use crate::traits::injectable::*;
pub use crate::traits::module::*;
