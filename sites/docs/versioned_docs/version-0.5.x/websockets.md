---
sidebar_position: 6
---

# WebSockets

WebSockets provide a persistent connection between a client and server, allowing for real-time, bidirectional communication. Ngyn provides built-in support for WebSockets through the `ngyn_websocket` crate.

## Setting Up WebSockets

To use WebSockets in your Ngyn application, you first need to add the WebSocket dependency to your `Cargo.toml`:

```toml
[dependencies]
ngyn = "0.5"
ngyn_websocket = "0.5"
tokio = { version = "1", features = ["full"] }
```

## Creating a WebSocket Server

Here's a basic example of setting up a WebSocket server with Ngyn:

```rust
use ngyn::prelude::*;
use ngyn_websocket::WebsocketApplication;

#[tokio::main]
async fn main() {
    let mut app = WebsocketApplication::default();

    // Define a route handler for the root path
    app.any("/", handler(|_| "WebSocket server is running"));

    println!("Starting WebSocket server at ws://127.0.0.1:8080");

    // Start the WebSocket server
    let _ = app.listen("0.0.0.0:8080");
}
```

## Client-Side Implementation

Here's a simple example of how to connect to your WebSocket server from a web browser:

```html
<!DOCTYPE html>
<html>
<head>
    <title>Ngyn WebSocket Client</title>
</head>
<body>
    <h1>Ngyn WebSocket Demo</h1>
    <div id="messages"></div>
    <input type="text" id="messageInput" placeholder="Type a message...">
    <button onclick="sendMessage()">Send</button>

    <script>
        const messagesDiv = document.getElementById('messages');
        const messageInput = document.getElementById('messageInput');
        const socket = new WebSocket('ws://localhost:8080');

        socket.onopen = function(e) {
            addMessage('Connected to server');
        };

        socket.onmessage = function(event) {
            addMessage(`Received: ${event.data}`);
        };

        socket.onclose = function(event) {
            if (event.wasClean) {
                addMessage(`Connection closed cleanly, code=${event.code} reason=${event.reason}`);
            } else {
                addMessage('Connection died');
            }
        };

        socket.onerror = function(error) {
            addMessage(`Error: ${error.message}`);
        };

        function sendMessage() {
            const message = messageInput.value;
            if (message) {
                socket.send(message);
                addMessage(`Sent: ${message}`);
                messageInput.value = '';
            }
        }

        function addMessage(message) {
            const messageElement = document.createElement('div');
            messageElement.textContent = message;
            messagesDiv.appendChild(messageElement);
        }
    </script>
</body>
</html>
```


For more advanced WebSocket examples, check out the [websocket example](https://github.com/ngyn-rs/ngyn/tree/main/examples/websocket) in the Ngyn repository.