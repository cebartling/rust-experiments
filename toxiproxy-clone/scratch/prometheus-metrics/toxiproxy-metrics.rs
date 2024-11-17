use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use lazy_static::lazy_static;
use prometheus::{
    register_counter_vec, register_histogram_vec, register_int_counter, 
    Counter, CounterVec, Histogram, HistogramVec, IntCounter,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;

// Metrics definitions
lazy_static! {
    static ref BYTES_TRANSFERRED: CounterVec = register_counter_vec!(
        "toxiproxy_bytes_transferred",
        "Total bytes transferred through proxy",
        &["proxy", "direction"]
    ).unwrap();

    static ref ACTIVE_CONNECTIONS: IntCounter = register_int_counter!(
        "toxiproxy_active_connections",
        "Number of active connections"
    ).unwrap();

    static ref TOXIC_ACTIVATIONS: CounterVec = register_counter_vec!(
        "toxiproxy_toxic_activations",
        "Number of times each toxic was activated",
        &["proxy", "toxic_type"]
    ).unwrap();

    static ref LATENCY_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "toxiproxy_latency_seconds",
        "Latency added by toxics",
        &["proxy", "toxic_type"],
        vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0]
    ).unwrap();

    static ref CORRUPTION_COUNTER: CounterVec = register_counter_vec!(
        "toxiproxy_corruptions",
        "Number of times data was corrupted",
        &["proxy"]
    ).unwrap();
}

// Metric collection structures
#[derive(Debug, Serialize)]
struct ProxyMetrics {
    bytes_upstream: f64,
    bytes_downstream: f64,
    active_connections: i64,
    toxic_activations: HashMap<String, u64>,
    latency_percentiles: HashMap<String, f64>,
    corruptions: u64,
}

// Existing Toxic trait with added metrics
trait Toxic: Send + Sync {
    fn modify_upstream(&self, data: &mut Vec<u8>, proxy_name: &str);
    fn modify_downstream(&self, data: &mut Vec<u8>, proxy_name: &str);
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
    fn modify_upstream(&self, _data: &mut Vec<u8>, proxy_name: &str) {
        let start = Instant::now();
        thread::sleep(self.latency);
        let duration = start.elapsed();
        
        TOXIC_ACTIVATIONS
            .with_label_values(&[proxy_name, "latency"])
            .inc();
        
        LATENCY_HISTOGRAM
            .with_label_values(&[proxy_name, "latency"])
            .observe(duration.as_secs_f64());
    }

    fn modify_downstream(&self, data: &mut Vec<u8>, proxy_name: &str) {
        self.modify_upstream(data, proxy_name);
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
    fn modify_upstream(&self, data: &mut Vec<u8>, proxy_name: &str) {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(self.probability) {
            if let Some(byte) = data.get_mut(0) {
                *byte = rng.gen();
                CORRUPTION_COUNTER.with_label_values(&[proxy_name]).inc();
            }
        }
    }

    fn modify_downstream(&self, data: &mut Vec<u8>, proxy_name: &str) {
        self.modify_upstream(data, proxy_name);
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

// Updated Proxy implementation with metrics
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

    fn proxy_data(
        mut from: TcpStream,
        mut to: TcpStream,
        toxics: Arc<Mutex<Vec<Arc<dyn Toxic>>>>,
        is_upstream: bool,
        proxy_name: String,
    ) {
        let direction = if is_upstream { "upstream" } else { "downstream" };
        let mut buffer = vec![0; 4096];

        while let Ok(n) = from.read(&mut buffer) {
            if n == 0 {
                break;
            }

            let mut data = buffer[..n].to_vec();
            
            // Record bytes transferred
            BYTES_TRANSFERRED
                .with_label_values(&[&proxy_name, direction])
                .inc_by(n as f64);

            // Apply toxics with metrics
            let toxics = toxics.lock().unwrap();
            for toxic in toxics.iter() {
                if is_upstream {
                    toxic.modify_upstream(&mut data, &proxy_name);
                } else {
                    toxic.modify_downstream(&mut data, &proxy_name);
                }
            }

            if to.write_all(&data).is_err() {
                break;
            }
        }

        ACTIVE_CONNECTIONS.dec();
    }

    fn handle_connection(
        upstream: TcpStream,
        downstream_addr: String,
        toxics: Arc<Mutex<Vec<Arc<dyn Toxic>>>>,
        proxy_name: String,
    ) -> io::Result<()> {
        ACTIVE_CONNECTIONS.inc();

        let downstream = TcpStream::connect(downstream_addr)?;
        let upstream_clone = upstream.try_clone()?;
        let downstream_clone = downstream.try_clone()?;

        let toxics_clone = Arc::clone(&toxics);
        let proxy_name_clone = proxy_name.clone();
        thread::spawn(move || {
            Self::proxy_data(
                upstream,
                downstream,
                toxics_clone,
                true,
                proxy_name_clone,
            );
        });

        let toxics_clone = Arc::clone(&toxics);
        thread::spawn(move || {
            Self::proxy_data(
                downstream_clone,
                upstream_clone,
                toxics_clone,
                false,
                proxy_name,
            );
        });

        Ok(())
    }
}

// Metrics endpoint handlers
async fn get_metrics() -> String {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();
    let mut buffer = Vec::new();
    encoder.encode(&prometheus::gather(), &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

async fn get_proxy_metrics(
    State(state): State<ProxyState>,
) -> Json<HashMap<String, ProxyMetrics>> {
    let state = state.lock().unwrap();
    let mut metrics = HashMap::new();

    for (proxy_name, _) in state.iter() {
        let bytes_upstream = BYTES_TRANSFERRED
            .with_label_values(&[proxy_name, "upstream"])
            .get();
        let bytes_downstream = BYTES_TRANSFERRED
            .with_label_values(&[proxy_name, "downstream"])
            .get();
        
        let mut toxic_activations = HashMap::new();
        for toxic_type in &["latency", "corrupt", "slow_close"] {
            toxic_activations.insert(
                toxic_type.to_string(),
                TOXIC_ACTIVATIONS
                    .with_label_values(&[proxy_name, toxic_type])
                    .get() as u64,
            );
        }

        let mut latency_percentiles = HashMap::new();
        let latency_metric = LATENCY_HISTOGRAM.with_label_values(&[proxy_name, "latency"]);
        for percentile in &[0.5, 0.9, 0.95, 0.99] {
            latency_percentiles.insert(
                format!("p{}", percentile * 100.0),
                latency_metric.get_sample_sum(),
            );
        }

        let corruptions = CORRUPTION_COUNTER
            .with_label_values(&[proxy_name])
            .get();

        metrics.insert(
            proxy_name.clone(),
            ProxyMetrics {
                bytes_upstream,
                bytes_downstream,
                active_connections: ACTIVE_CONNECTIONS.get(),
                toxic_activations,
                latency_percentiles,
                corruptions,
            },
        );
    }

    Json(metrics)
}

// Main router setup with metrics endpoints
#[tokio::main]
async fn main() -> io::Result<()> {
    let proxy_state: ProxyState = Arc::new(Mutex::new(HashMap::new()));
    
    let app = Router::new()
        .route("/metrics", get(get_metrics))
        .route("/metrics/proxy", get(get_proxy_metrics))
        .route("/toxics", get(list_toxics))
        .route("/toxics", post(add_toxic))
        .layer(CorsLayer::permissive())
        .with_state(Arc::clone(&proxy_state));

    println!("Starting server with metrics endpoints...");
    println!("Prometheus metrics available at: http://localhost:8474/metrics");
    println!("Proxy metrics available at: http://localhost:8474/metrics/proxy");

    axum::Server::bind(&"127.0.0.1:8474".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
