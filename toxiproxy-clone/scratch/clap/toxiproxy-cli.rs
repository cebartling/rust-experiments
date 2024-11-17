use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use lazy_static::lazy_static;
use prometheus::{
    register_counter_vec, register_histogram_vec, register_int_counter, 
    CounterVec, HistogramVec, IntCounter,
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

// CLI Arguments
#[derive(Parser, Debug)]
#[command(
    name = "toxiproxy",
    about = "A TCP proxy for simulating network conditions",
    version
)]
struct Args {
    /// Port for the REST API
    #[arg(long, default_value = "8474")]
    api_port: u16,

    /// Port the proxy listens on
    #[arg(long, default_value = "8475")]
    proxy_port: u16,

    /// Port of the upstream service
    #[arg(long, default_value = "8476")]
    upstream_port: u16,

    /// Host address to bind to
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// Upstream host address
    #[arg(long, default_value = "127.0.0.1")]
    upstream_host: String,
}

// Rest of the existing metrics definitions...
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

    // ... other metrics remain the same
}

// Modified main function to use CLI arguments
#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    
    let api_address = format!("{}:{}", args.host, args.api_port);
    let proxy_address = format!("{}:{}", args.host, args.proxy_port);
    let upstream_address = format!("{}:{}", args.upstream_host, args.upstream_port);

    let proxy_state: ProxyState = Arc::new(Mutex::new(HashMap::new()));
    
    // Create shutdown channel
    let (shutdown_tx, _) = broadcast::channel(1);

    // Setup REST API
    let app = Router::new()
        .route("/metrics", get(get_metrics))
        .route("/metrics/proxy", get(get_proxy_metrics))
        .route("/toxics", get(list_toxics))
        .route("/toxics", post(add_toxic))
        .layer(CorsLayer::permissive())
        .with_state(Arc::clone(&proxy_state));

    println!("Starting Toxiproxy...");
    println!("REST API listening on: {}", api_address);
    println!("Proxy listening on: {}", proxy_address);
    println!("Forwarding to upstream: {}", upstream_address);
    println!("Prometheus metrics available at: http://{}/metrics", api_address);
    println!("Proxy metrics available at: http://{}/metrics/proxy", api_address);

    // Start proxy
    let proxy = Proxy::new("main".to_string());
    let proxy_handle = {
        let proxy_address = proxy_address.clone();
        let upstream_address = upstream_address.clone();
        tokio::spawn(async move {
            if let Err(e) = proxy
                .start(&proxy_address, &upstream_address, shutdown_tx.subscribe())
                .await
            {
                eprintln!("Proxy error: {}", e);
            }
        })
    };

    // Start API server
    let server = axum::Server::bind(&api_address.parse().unwrap())
        .serve(app.into_make_service());

    println!("\nUse the following command to add a latency toxic:");
    println!("curl -X POST http://{}/toxics \\", api_address);
    println!(r#"  -H "Content-Type: application/json" \\"#);
    println!(r#"  -d '{{"proxy": "main", "config": {{"type": "Latency", "latency_ms": 100}}}}'"#);

    // Wait for both tasks
    tokio::select! {
        _ = server => println!("API server stopped"),
        _ = proxy_handle => println!("Proxy stopped"),
    }

    Ok(())
}

// Helper function to display usage examples
fn print_usage_examples(api_address: &str) {
    println!("\nExample commands:");
    
    println!("\n1. Add latency toxic:");
    println!("curl -X POST http://{}/toxics \\", api_address);
    println!(r#"  -H "Content-Type: application/json" \\"#);
    println!(r#"  -d '{{"proxy": "main", "config": {{"type": "Latency", "latency_ms": 100}}}}'"#);
    
    println!("\n2. Add corruption toxic:");
    println!("curl -X POST http://{}/toxics \\", api_address);
    println!(r#"  -H "Content-Type: application/json" \\"#);
    println!(r#"  -d '{{"proxy": "main", "config": {{"type": "Corrupt", "probability": 0.01}}}}'"#);
    
    println!("\n3. View metrics:");
    println!("curl http://{}/metrics", api_address);
}

// Rest of the existing code remains the same...
