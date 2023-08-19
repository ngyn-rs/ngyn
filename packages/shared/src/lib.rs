pub mod traits {
    pub mod controller_trait;
    pub mod injectable_trait;
    pub mod module_trait;
}

pub mod common {
    pub mod provider;
}

pub use crate::common::provider::*;
pub use crate::traits::controller_trait::*;
pub use crate::traits::injectable_trait::*;
pub use crate::traits::module_trait::*;
