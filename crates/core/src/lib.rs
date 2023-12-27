pub mod app;
pub mod platforms;

pub mod macros {
    pub use async_std::main;
    pub use ngyn_macros::*;
    pub use nject::injectable as dependency;
}

pub mod prelude {
    pub use crate::app::*;
    pub use crate::macros::*;
    pub use ngyn_shared::*;
}
