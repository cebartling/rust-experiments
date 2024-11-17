I'll add a REST API to dynamically configure toxics using the `axum` web framework.



```rust
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;

// Existing Toxic trait and implementations
trait Toxic: Send + Sync {
    fn modify_upstream(&self, data: &mut Vec<u8>);
    fn modify_downstream(&self, data: &mut Vec<u8>);
    fn get_type(&self) -> String;
    fn get_config(&self) -> ToxicConfig;
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum ToxicConfig {
    Latency { latency_ms: u64 },
    Corrupt { probability: f64 },
    SlowClose { delay_ms: u64 },
}

struct LatencyToxic {
    latency: Duration,
}

impl Toxic for LatencyToxic {
    fn modify_upstream(&self, _data: &mut Vec<u8>) {
        thread::sleep(self.latency);
    }

    fn modify_downstream(&self, _data: &mut Vec<u8>) {
        thread::sleep(self.latency);
    }

    fn get_type(&self) -> String {
        "latency".to_string()
    }

    fn get_config(&self) -> ToxicConfig {
        ToxicConfig::Latency {
            latency_ms: self.latency.as_millis() as u64,
        }
    }
}

struct CorruptToxic {
    probability: f64,
}

impl Toxic for CorruptToxic {
    fn modify_upstream(&self, data: &mut Vec<u8>) {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(self.probability) {
            if let Some(byte) = data.get_mut(0) {
                *byte = rng.gen();
            }
        }
    }

    fn modify_downstream(&self, data: &mut Vec<u8>) {
        self.modify_upstream(data);
    }

    fn get_type(&self) -> String {
        "corrupt".to_string()
    }

    fn get_config(&self) -> ToxicConfig {
        ToxicConfig::Corrupt {
            probability: self.probability,
        }
    }
}

struct SlowCloseToxic {
    delay: Duration,
}

impl Toxic for SlowCloseToxic {
    fn modify_upstream(&self, _data: &mut Vec<u8>) {}
    fn modify_downstream(&self, _data: &mut Vec<u8>) {}

    fn get_type(&self) -> String {
        "slow_close".to_string()
    }

    fn get_config(&self) -> ToxicConfig {
        ToxicConfig::SlowClose {
            delay_ms: self.delay.as_millis() as u64,
        }
    }
}

// Proxy state and configuration types
type ProxyState = Arc<Mutex<HashMap<String, Vec<Arc<dyn Toxic>>>>>;

#[derive(Debug, Serialize, Deserialize)]
struct ProxyConfig {
    name: String,
    listen: String,
    upstream: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateToxicRequest {
    proxy: String,
    config: ToxicConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct ToxicResponse {
    proxy: String,
    toxic_type: String,
    config: ToxicConfig,
}

// REST API handlers
async fn list_toxics(
    State(state): State<ProxyState>,
) -> Json<Vec<ToxicResponse>> {
    let state = state.lock().unwrap();
    let mut responses = Vec::new();

    for (proxy_name, toxics) in state.iter() {
        for toxic in toxics {
            responses.push(ToxicResponse {
                proxy: proxy_name.clone(),
                toxic_type: toxic.get_type(),
                config: toxic.get_config(),
            });
        }
    }

    Json(responses)
}

async fn add_toxic(
    State(state): State<ProxyState>,
    Json(request): Json<CreateToxicRequest>,
) -> Json<ToxicResponse> {
    let toxic: Arc<dyn Toxic> = match request.config {
        ToxicConfig::Latency { latency_ms } => Arc::new(LatencyToxic {
            latency: Duration::from_millis(latency_ms),
        }),
        ToxicConfig::Corrupt { probability } => Arc::new(CorruptToxic { probability }),
        ToxicConfig::SlowClose { delay_ms } => Arc::new(SlowCloseToxic {
            delay: Duration::from_millis(delay_ms),
        }),
    };

    let mut state = state.lock().unwrap();
    let toxics = state
        .entry(request.proxy.clone())
        .or_insert_with(Vec::new);
    
    let response = ToxicResponse {
        proxy: request.proxy,
        toxic_type: toxic.get_type(),
        config: toxic.get_config(),
    };

    toxics.push(toxic);
    Json(response)
}

// Proxy implementation
struct Proxy {
    name: String,
    toxics: Arc<Mutex<Vec<Arc<dyn Toxic>>>>,
}

impl Proxy {
    fn new(name: String) -> Self {
        Proxy {
            name,
            toxics: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn handle_connection(
        upstream: TcpStream,
        downstream_addr: String,
        toxics: Arc<Mutex<Vec<Arc<dyn Toxic>>>>,
    ) -> io::Result<()> {
        let downstream = TcpStream::connect(downstream_addr)?;
        let upstream_clone = upstream.try_clone()?;
        let downstream_clone = downstream.try_clone()?;

        let toxics_clone = Arc::clone(&toxics);
        thread::spawn(move || {
            Self::proxy_data(upstream, downstream, toxics_clone, true);
        });

        let toxics_clone = Arc::clone(&toxics);
        thread::spawn(move || {
            Self::proxy_data(downstream_clone, upstream_clone, toxics_clone, false);
        });

        Ok(())
    }

    fn proxy_data(
        mut from: TcpStream,
        mut to: TcpStream,
        toxics: Arc<Mutex<Vec<Arc<dyn Toxic>>>>,
        is_upstream: bool,
    ) {
        let mut buffer = vec![0; 4096];
        loop {
            match from.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    let mut data = buffer[..n].to_vec();
                    
                    // Apply toxics
                    let toxics = toxics.lock().unwrap();
                    for toxic in toxics.iter() {
                        if is_upstream {
                            toxic.modify_upstream(&mut data);
                        } else {
                            toxic.modify_downstream(&mut data);
                        }
                    }

                    if let Err(_) = to.write_all(&data) {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    }

    async fn start(
        &self,
        listen_addr: &str,
        upstream_addr: &str,
        shutdown: broadcast::Receiver<()>,
    ) -> io::Result<()> {
        let listener = TcpListener::bind(listen_addr)?;
        listener.set_nonblocking(true)?;
        
        println!("Proxy {} listening on {}", self.name, listen_addr);
        println!("Forwarding to {}", upstream_addr);

        let toxics = Arc::clone(&self.toxics);
        let upstream_addr = upstream_addr.to_string();

        tokio::spawn(async move {
            loop {
                match listener.accept() {
                    Ok((stream, _)) => {
                        let upstream_addr = upstream_addr.clone();
                        let toxics = Arc::clone(&toxics);
                        thread::spawn(move || {
                            if let Err(e) = Self::handle_connection(stream, upstream_addr, toxics) {
                                eprintln!("Connection error: {}", e);
                            }
                        });
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(e) => eprintln!("Accept failed: {}", e),
                }
            }
        });

        Ok(())
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let proxy_state: ProxyState = Arc::new(Mutex::new(HashMap::new()));
    
    // Create shutdown channel
    let (shutdown_tx, _) = broadcast::channel(1);

    // Setup REST API
    let app = Router::new()
        .route("/toxics", get(list_toxics))
        .route("/toxics", post(add_toxic))
        .layer(CorsLayer::permissive())
        .with_state(Arc::clone(&proxy_state));

    // Start API server
    let api_handle = tokio::spawn(async move {
        axum::Server::bind(&"127.0.0.1:8474".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    // Create and start proxy
    let proxy = Proxy::new("main".to_string());
    proxy.start("127.0.0.1:8475", "127.0.0.1:8476", shutdown_tx.subscribe()).await?;

    api_handle.await.unwrap();
    Ok(())
}

```

I've implemented a REST API for the Toxiproxy clone. Here are the key additions and changes:

1. **REST API Endpoints**:
    - `GET /toxics` - Lists all configured toxics across all proxies
    - `POST /toxics` - Adds a new toxic to a proxy

2. **Configuration Types**:
    - `ToxicConfig` enum for different toxic types
    - `CreateToxicRequest` for adding new toxics
    - `ToxicResponse` for API responses

3. **Dynamic Configuration**:
    - Toxics can be added at runtime via the API
    - State is managed using `Arc<Mutex<HashMap>>` for thread-safe access

Here's how to use it:

1. Start the server:
```bash
cargo run
```

2. Add a new latency toxic:
```bash
curl -X POST http://localhost:8474/toxics \
  -H "Content-Type: application/json" \
  -d '{
    "proxy": "main",
    "config": {
      "type": "Latency",
      "latency_ms": 100
    }
  }'
```

3. List all toxics:
```bash
curl http://localhost:8474/toxics
```

Example toxic configurations:

```json
// Latency toxic
{
  "proxy": "main",
  "config": {
    "type": "Latency",
    "latency_ms": 100
  }
}

// Corrupt toxic
{
  "proxy": "main",
  "config": {
    "type": "Corrupt",
    "probability": 0.01
  }
}

// Slow close toxic
{
  "proxy": "main",
  "config": {
    "type": "SlowClose",
    "delay_ms": 1000
  }
}
```

Would you like me to:
1. Add toxic removal functionality?
2. Add proxy configuration endpoints (create/delete proxies)?
3. Add metrics collection and monitoring endpoints?