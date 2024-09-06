use ngyn_shared::{
    core::{NgynPlatform, PlatformData},
    server::response::ReadBytes,
};
use vercel_runtime::{Body, Error, Request, Response as VercelResponse};

#[derive(Default)]
pub struct VercelApplication {
    data: PlatformData,
}

impl NgynPlatform for VercelApplication {
    fn data_mut(&mut self) -> &mut PlatformData {
        &mut self.data
    }
}

impl VercelApplication {
    pub async fn handle(self, request: Request) -> Result<VercelResponse<Body>, Error> {
        let request = request.map(|b| b.to_vec());
        let mut res = self.data.respond(request).await;

        let body = res.read_bytes().await.unwrap(); // infallible, only fails if the response is invalid

        let (parts, ..) = res.into_parts();

        let response = VercelResponse::from_parts(parts, Body::from(body.to_vec()));
        Ok(response)
    }
}
