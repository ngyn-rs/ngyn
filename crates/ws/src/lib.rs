use core::fmt;
use ngyn_shared::core::engine::{NgynPlatform, PlatformData};
use ngyn_shared::core::handler::RouteHandler;
use ngyn_shared::server::response::ReadBytes;
use ngyn_shared::server::NgynRequest;
use std::io::ErrorKind;
use std::net::ToSocketAddrs;
use std::sync::{Arc, Mutex};
use websocket::Message;
use websocket::{sync::Client, sync::Server, OwnedMessage};

#[derive(Default)]
pub struct WebsocketApplication {
    data: PlatformData,
    clients: Arc<Mutex<Vec<Client<std::net::TcpStream>>>>,
}

impl NgynPlatform for WebsocketApplication {
    fn data_mut(&mut self) -> &mut PlatformData {
        &mut self.data
    }
}

impl WebsocketApplication {
    /// add a route to handle
    pub fn route(&mut self, path: &str, handler: impl Into<RouteHandler>) {
        self.data_mut().add_route(path, None, handler.into());
    }

    // Broadcast message to all connected clients
    pub fn broadcast(&self, message: &str) -> Result<(), websocket::WebSocketError> {
        let mut clients = self
            .clients
            .lock()
            .map_err(|_| websocket::WebSocketError::IoError(ErrorKind::InvalidData.into()))?;

        for client in clients.iter_mut() {
            client.send_message(&OwnedMessage::Text(message.to_string()))?;
        }

        Ok(())
    }

    /// Listens for incoming connections and serves the application.
    ///
    /// ### Arguments
    ///
    /// * `addr` - The address to listen on.
    ///
    /// ### Returns
    ///
    /// A `Result` indicating success or failure.
    pub fn listen<A: ToSocketAddrs + fmt::Debug>(
        self,
        addr: A,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let server = Server::bind(addr)?;
        let data_handler = Arc::new(self.data);

        for request in server.filter_map(Result::ok) {
            let clients = Arc::clone(&self.clients);
            let data_handler = data_handler.clone();

            tokio::spawn(async move {
                if let Ok(mut client) = request.accept() {
                    for message in client.incoming_messages() {
                        match message {
                            Ok(OwnedMessage::Text(msg)) => {
                                let (path, body) =
                                    msg.split_once(':').unwrap_or_else(|| ("/", &msg));

                                let mut req = NgynRequest::new(body.into());
                                if let Ok(uri) = path.parse() {
                                    *req.uri_mut() = uri;
                                    let mut response = data_handler.respond(req).await;
                                    if let Ok(data) = response.read_bytes().await {
                                        let message = Message::binary(data.to_vec());
                                        client.send_message(&message).unwrap();
                                    }
                                    return;
                                }
                            }
                            Ok(OwnedMessage::Close(_)) => {
                                let message = Message::close();
                                client.send_message(&message).unwrap();
                                break;
                            }
                            Ok(OwnedMessage::Ping(data)) => {
                                let message = Message::pong(data);
                                client.send_message(&message).unwrap();
                                break;
                            }
                            Err(_) => break,
                            _ => {}
                        }
                    }

                    // Add client to the list of connected clients
                    if let Ok(mut client_list) = clients.lock() {
                        client_list.push(client);
                    }
                }
            });
        }

        Ok(())
    }
}
