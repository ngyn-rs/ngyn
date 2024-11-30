use ngyn_shared::{
    core::engine::{NgynPlatform, PlatformData},
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
        let mut response = self.data.respond(request).await;

        let body = response
            .read_bytes()
            .await
            .expect("Response hasn't been set");

        let (parts, ..) = response.into_parts();

        Ok(VercelResponse::from_parts(parts, Body::from(body.to_vec())))
    }
}
