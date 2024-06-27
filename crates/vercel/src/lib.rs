use http_body_util::BodyExt;
use ngyn_shared::{
    core::{NgynEngine, PlatformData},
    server::{response::ResponseBuilder, NgynResponse},
};
use std::sync::Arc;
use vercel_runtime::{Body, Error, Request, Response as VercelResponse};

#[derive(Default)]
pub struct VercelApplication {
    data: PlatformData,
}

impl NgynEngine for VercelApplication {
    fn data_mut(&mut self) -> &mut PlatformData {
        &mut self.data
    }
}

impl VercelApplication {
    pub async fn handle(self, request: Request) -> Result<VercelResponse<Body>, Error> {
        let request = request.map(|b| b.to_vec());

        let response = NgynResponse::build(
            request,
            Arc::new(self.data.routes),
            Arc::new(self.data.middlewares),
        )
        .await;

        let (parts, mut body) = response.into_parts();
        let body = {
            let mut buf = Vec::new();
            let frame = body.frame().await;
            if frame.is_some() {
                let chunk = frame.unwrap().unwrap();
                let d = chunk.data_ref().unwrap();
                buf.extend_from_slice(d.to_vec().as_slice());
            }
            Body::from(buf)
        };

        let response = VercelResponse::from_parts(parts, body);

        Ok(response)
    }
}
