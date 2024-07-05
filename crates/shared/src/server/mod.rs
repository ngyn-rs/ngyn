pub mod body;
pub mod context;
pub mod response;
pub mod transformer;
pub mod uri;

pub use self::response::FullResponse;
pub use body::{ParseBytes, ToBytes};
pub use context::NgynContext;
use http_body_util::Full;
pub use hyper::body::Bytes;
pub use transformer::{Body, Param, Query, Transducer, Transformer};

pub type NgynRequest = hyper::Request<Vec<u8>>;
pub type NgynResponse = hyper::Response<Full<Bytes>>;

pub use hyper::http::Method;
