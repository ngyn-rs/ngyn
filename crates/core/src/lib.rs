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
    ($res:expr, $status_code:expr) => {
        *$res.status_mut() = $status_code;
        return ngyn::prelude::Bytes::from("".to_string()).parse_bytes();
    };
    ($res:expr, $status_code:expr, $message:expr) => {
        *$res.status_mut() = ngyn::http::StatusCode::try_from($status_code).unwrap();
        return ngyn::prelude::Bytes::from($message).parse_bytes();
    };
}
