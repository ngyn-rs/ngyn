use ngyn_macros::platform;
use ngyn_shared::{Handler, HttpMethod, NgynBody, NgynEngine, NgynRequest, NgynResponse};
use std::{collections::HashMap, sync::Arc};
use tide::{Response, Server};

pub type Result<T = Response> = tide::Result<T>;

/// `NgynApplication` is a struct that represents a server instance in the Ngyn framework.
#[platform]
pub struct NgynApplication {
    server: Server<()>,
}

impl NgynApplication {
    fn build(res: NgynResponse) -> Result {
        let mut response = Response::new(res.status());

        match res.body_raw() {
            NgynBody::String(body) => response.set_body(body),
            NgynBody::Bool(body) => response.set_body(body.to_string()),
            NgynBody::Number(body) => response.set_body(body.to_string()),
            NgynBody::Map(body) => {
                let mut body_string = String::new();
                for (key, value) in body {
                    let value_str: String = value.parse();
                    body_string.push_str(&format!("{}: {}\n", key, value_str));
                }
                response.set_body(body_string);
            }
            NgynBody::None => {}
        }

        for header in res.headers() {
            let mut header = header.split(':');
            let key = header.next().unwrap_or("").trim();
            let value = header.next().unwrap_or("").trim();

            response.insert_header(key, value);
        }

        Ok(response)
    }

    /// Starts listening for incoming connections on the specified address.
    /// This function is asynchronous and returns a `tide::Result`.
    pub async fn listen(self, address: &str) -> Result<()> {
        self.server.listen(address).await.map_err(tide::Error::from)
    }
}

impl NgynEngine for NgynApplication {
    fn new() -> Self {
        Self {
            server: Server::new(),
        }
    }

    fn route(&mut self, path: &str, method: HttpMethod, handler: Box<impl Handler>) -> &mut Self {
        let handler = Arc::new(handler);
        let req_handler = {
            let handler = Arc::clone(&handler);
            move |req: tide::Request<()>| {
                let handler = Arc::clone(&handler);
                async move {
                    let values = request_to_values(req).await;
                    let mut request = NgynRequest::from(values);
                    let mut response = NgynResponse::from_status(200);
                    handler.handle(&mut request, &mut response);
                    Self::build(response.await)
                }
            }
        };
        match method {
            HttpMethod::Get => self.server.at(path).get(req_handler),
            HttpMethod::Post => self.server.at(path).post(req_handler),
            HttpMethod::Put => self.server.at(path).put(req_handler),
            HttpMethod::Delete => self.server.at(path).delete(req_handler),
            HttpMethod::Patch => self.server.at(path).patch(req_handler),
            HttpMethod::Head => self.server.at(path).head(req_handler),
            _ => panic!("Unsupported HTTP method"),
        };
        self
    }
}

async fn request_to_values(
    mut request: tide::Request<()>,
) -> (String, String, HashMap<String, String>, Vec<u8>) {
    let method = request.method().to_string();
    let url = request.url().to_string();
    let headers = {
        let mut headers_map = HashMap::new();
        for name in request.header_names() {
            if let Some(value) = request.header(name.as_str()) {
                headers_map.insert(name.to_string(), value.last().to_string());
            }
        }
        headers_map
    };
    let body = request.body_bytes().await.unwrap();
    (method, url, headers, body)
}
