---
sidebar_position: 9
---

# Deployment

Deploying your Ngyn application to production requires careful planning and consideration of various factors such as performance, security, and reliability. This guide will walk you through the process of deploying your Ngyn application to different environments.

## Building for Production

Before deploying your Ngyn application, you should build it in release mode to optimize performance:

```bash
cargo build --release
```

This will create an optimized binary in the `target/release` directory.

## Deployment Options

### Self-Hosting

Self-hosting gives you complete control over your application and infrastructure. Here's how to deploy a Ngyn application on a Linux server:

1. **Transfer the binary to your server**:

   ```bash
   scp target/release/your_app user@your-server:/path/to/destination
   ```

2. **Set up a systemd service** (on systemd-based Linux distributions):

   Create a service file at `/etc/systemd/system/your-app.service`:

   ```ini
   [Unit]
   Description=Your Ngyn Application
   After=network.target

   [Service]
   User=your-user
   WorkingDirectory=/path/to/your/app
   ExecStart=/path/to/your/app/your_app
   Restart=always
   RestartSec=5
   Environment=RUST_LOG=info

   [Install]
   WantedBy=multi-user.target
   ```

3. **Enable and start the service**:

   ```bash
   sudo systemctl enable your-app
   sudo systemctl start your-app
   ```

4. **Set up a reverse proxy** (optional but recommended):

   Using Nginx:

   ```nginx
   server {
       listen 80;
       server_name your-domain.com;

       location / {
           proxy_pass http://127.0.0.1:3000;
           proxy_http_version 1.1;
           proxy_set_header Upgrade $http_upgrade;
           proxy_set_header Connection 'upgrade';
           proxy_set_header Host $host;
           proxy_cache_bypass $http_upgrade;
       }
   }
   ```

### Docker Deployment

Docker provides a consistent environment for your application. Here's how to containerize your Ngyn application:

1. **Create a Dockerfile**:

   ```dockerfile
   FROM rust:1.70 as builder
   WORKDIR /usr/src/app
   COPY . .
   RUN cargo build --release

   FROM debian:bullseye-slim
   RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
   COPY --from=builder /usr/src/app/target/release/your_app /usr/local/bin/your_app
   EXPOSE 3000
   CMD ["your_app"]
   ```

2. **Build the Docker image**:

   ```bash
   docker build -t your-app:latest .
   ```

3. **Run the container**:

   ```bash
   docker run -p 3000:3000 your-app:latest
   ```

4. **For production, you might want to use Docker Compose**:

   Create a `docker-compose.yml` file:

   ```yaml
   version: '3'
   services:
     app:
       image: your-app:latest
       ports:
         - "3000:3000"
       restart: always
       environment:
         - RUST_LOG=info
   ```

   Then run:

   ```bash
   docker-compose up -d
   ```

### Cloud Platforms

#### Vercel

Ngyn has built-in support for Vercel through the `ngyn_vercel` crate:

1. **Add the Vercel dependency**:

   ```toml
   [dependencies]
   ngyn = "0.5"
   ngyn_vercel = "0.5"
   ```

2. **Create a Vercel-compatible application**:

   ```rust
   use ngyn::prelude::*;
   use ngyn_vercel::VercelApplication;

   #[handler]
   fn hello() -> &'static str {
       "Hello from Vercel!"
   }

   #[no_mangle]
   pub fn vercel_handler() {
       let mut app = VercelApplication::default();
       app.get("/api/hello", hello);
       app.start();
   }
   ```

3. **Configure your `vercel.json`**:

   ```json
   {
     "version": 2,
     "functions": {
       "api/**/*": {
         "runtime": "vercel-rust@4.0.0"
       }
     },
     "routes": [
       { "src": "/api/(.*)", "dest": "/api/$1" }
     ]
   }
   ```

4. **Deploy to Vercel**:

   ```bash
   vercel
   ```

#### Shuttle

Shuttle is a Rust-native cloud platform that makes deploying Rust applications easy:

1. **Add the Shuttle dependency**:

   ```toml
   [dependencies]
   ngyn = "0.5"
   ngyn_shuttle = "0.5"
   shuttle-runtime = "0.19.0"
   ```

2. **Create a Shuttle-compatible application**:

   ```rust
   use ngyn::prelude::*;
   use ngyn_shuttle::{ShuttleApplication, ShuttleNgyn};

   #[handler]
   fn hello() -> &'static str {
       "Hello from Shuttle!"
   }

   #[shuttle_runtime::main]
   async fn main() -> ShuttleNgyn {
       let mut app = ShuttleApplication::default();
       app.get("/", hello);
       Ok(app.into())
   }
   ```

3. **Deploy to Shuttle**:

   ```bash
   cargo shuttle deploy
   ```

## Production Considerations

### Environment Variables

Use environment variables for configuration in production:

```rust
use std::env;

fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // Use these variables in your application
}
```

### Logging

Implement proper logging for production environments:

```rust
use env_logger::Env;

fn main() {
    // Initialize logger with default level of info
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();
    
    // Your application code
}
```

### Health Checks

Implement health check endpoints to monitor your application:

```rust
#[handler]
fn health_check() -> JsonResult {
    Ok(json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

// Register the health check endpoint
app.get("/health", health_check);
```

### HTTPS

Always use HTTPS in production. If you're using a reverse proxy like Nginx, you can configure SSL/TLS there. Alternatively, you can implement HTTPS directly in your Ngyn application:

```rust
use std::path::PathBuf;
use ngyn::prelude::*;
use rustls::{Certificate, PrivateKey, ServerConfig};
use tokio_rustls::TlsAcceptor;

async fn main() {
    let cert_path = PathBuf::from("path/to/cert.pem");
    let key_path = PathBuf::from("path/to/key.pem");
    
    // Load certificates and key
    let cert = std::fs::read(&cert_path).expect("Failed to read certificate");
    let key = std::fs::read(&key_path).expect("Failed to read private key");
    
    // Configure TLS
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(vec![Certificate(cert)], PrivateKey(key))
        .expect("Failed to configure TLS");
    
    let acceptor = TlsAcceptor::from(std::sync::Arc::new(config));
    
    // Your Ngyn application setup
    let mut app = HyperApplication::default();
    // Register routes
    
    // Start the server with TLS
    let _ = app.listen_tls("0.0.0.0:443", acceptor).await;
}
```

## Monitoring and Scaling

### Prometheus Metrics

Implement Prometheus metrics to monitor your application:

```rust
use prometheus::{Counter, Registry};

// Create a registry
let registry = Registry::new();

// Create a counter
let request_counter = Counter::new("http_requests_total", "Total HTTP Requests").unwrap();
registry.register(Box::new(request_counter.clone())).unwrap();

// Create a middleware to count requests
struct MetricsMiddleware {
    counter: Counter,
}

impl NgynMiddleware for MetricsMiddleware {
    async fn handle(ctx: NgynContext) {
        self.counter.inc();
    }
}

// Add the middleware to your application
app.use_middleware(MetricsMiddleware { counter: request_counter });

// Add a metrics endpoint
app.get("/metrics", handler(move |_| {
    let mut buffer = Vec::new();
    let encoder = prometheus::TextEncoder::new();
    encoder.encode(&registry.gather(), &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}));
```

### Load Balancing

For high-traffic applications, consider using a load balancer to distribute traffic across multiple instances of your application:

1. **Run multiple instances** of your application on different ports
2. **Configure a load balancer** (like Nginx, HAProxy, or a cloud load balancer) to distribute traffic

```nginx
# Example Nginx load balancer configuration
upstream backend {
    server 127.0.0.1:3001;
    server 127.0.0.1:3002;
    server 127.0.0.1:3003;
}

server {
    listen 80;
    server_name your-domain.com;

    location / {
        proxy_pass http://backend;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

For more advanced deployment scenarios, check out the [examples](https://github.com/ngyn-rs/ngyn/tree/main/examples) in the Ngyn repository.