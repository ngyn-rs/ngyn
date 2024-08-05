use http_body_util::BodyExt;
use ngyn_shared::core::{NgynPlatform, PlatformData};
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
        let response = self.data.respond(request).await;

        let (parts, mut body) = response.into_parts();
        std::str::from_utf8(
            &body
                .frame()
                .await
                .unwrap()
                .unwrap()
                .data_ref()
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        let body = {
            let mut buf = Vec::new();
            let frame = body.frame().await;

            match frame {
                Some(frame) => {
                    if let Ok(chunk) = frame {
                        let d = chunk.data_ref().unwrap();
                        buf.extend_from_slice(d.to_vec().as_slice());
                    }
                }
                None => {}
            }
            Body::from(buf)
        };

        let response = VercelResponse::from_parts(parts, body);
        Ok(response)
    }
}
