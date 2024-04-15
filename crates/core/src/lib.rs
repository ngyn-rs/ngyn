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
    pub use crate::eject;
    pub use crate::macros::*;
    pub use ngyn_shared::*;
}

pub mod http {
    pub use hyper::http::*;
}

/// The `eject` macro is used to return an HTTP response with a given status code and optional message.
///
/// # Examples
///
/// ```rust ignore
/// # use ngyn::prelude::*;
///
/// fn handle_request() -> Bytes {
///     let status_code = 404;
///     let message = "Not Found";
///     eject!(status_code, message)
/// }
/// ```
#[macro_export]
macro_rules! eject {
    ($status_code:expr) => {
        return ngyn::prelude::Bytes::from($status_code.to_string()).parse_bytes();
    };
    ($status_code:expr, $message:expr) => {
        return ngyn::prelude::Bytes::from(format!("{} - {}", $status_code, $message)).parse_bytes();
    };
}
