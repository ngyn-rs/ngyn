pub mod body;
pub mod context;
pub mod response;
pub mod transformer;
pub mod uri;

pub use self::response::FullResponse;
pub use body::{ParseBody, ParseBytes};
pub use context::NgynContext;
use http_body_util::Full;
pub use hyper::body::Bytes;
pub use transformer::{Dto, Param, Query, Transducer, Transformer};
pub use uri::ToParams;

pub type NgynRequest = hyper::Request<hyper::body::Incoming>;
pub type NgynResponse = hyper::Response<Full<Bytes>>;
