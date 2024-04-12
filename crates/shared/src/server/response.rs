use http_body_util::Full;
use hyper::StatusCode;

use crate::{context::NgynContext, transformer::Transformer, NgynResponse, ParseBody};

pub trait FullResponse {
    /**
     * Set the status code of the response
     */
    fn set_status(&mut self, status: u16) -> &mut Self;

    /**
     * Peek into the response body
     */
    fn peek(&mut self, item: impl ParseBody) -> &mut Self;
}

impl FullResponse for NgynResponse {
    fn set_status(&mut self, status: u16) -> &mut Self {
        *self.status_mut() = StatusCode::from_u16(status).unwrap();
        self
    }

    fn peek(&mut self, item: impl ParseBody) -> &mut Self {
        *self.body_mut() = Full::new(item.parse_body());
        self
    }
}

impl Transformer for NgynResponse {
    fn transform(_cx: &mut NgynContext, res: &mut NgynResponse) -> Self {
        res.clone()
    }
}
