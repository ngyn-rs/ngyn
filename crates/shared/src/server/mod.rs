pub mod body;
pub mod context;
pub mod response;
pub mod transformer;
pub mod uri;

pub use self::response::{CommonResponse, FullResponse};
pub use body::ToBytes;
pub use context::NgynContext;
use http_body_util::Full;
pub use hyper::{body::Bytes, http::Method};
pub use transformer::{Body, Param, Query, Transducer, Transformer};

pub type NgynRequest = hyper::Request<Vec<u8>>;
pub type NgynResponse = hyper::Response<Full<Bytes>>;

pub(crate) type Routes = Vec<(String, Method, Box<crate::core::Handler>)>;
pub(crate) type Middlewares = Vec<Box<dyn crate::traits::NgynMiddleware>>;
