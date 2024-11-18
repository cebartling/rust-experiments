mod args;
mod proxy;
mod rest_api;
mod toxic;
mod toxics;

use crate::args::Args;
use crate::proxy::Proxy;
use crate::toxics::corrupt::CorruptToxic;
use crate::toxics::latency::LatencyToxic;
use crate::toxics::slow_close::SlowCloseToxic;
use clap::Parser;
use std::{
    io::{self},
    sync::Arc,
    time::Duration,
};

fn main() -> io::Result<()> {
    let args = Args::parse();

    // let api_address = format!("{}:{}", args.host, args.api_port);
    let proxy_address = format!("{}:{}", args.host, args.proxy_port);
    let upstream_address = format!("{}:{}", args.upstream_host, args.upstream_port);

    // let proxy_state: ProxyState = Arc::new(Mutex::new(HashMap::new()));

    println!("Starting Toxiproxy...");
    // println!("REST API listening on: {}", api_address);
    println!("Proxy listening on: {}", proxy_address);
    println!("Forwarding to upstream: {}", upstream_address);
    // println!("Prometheus metrics available at: http://{}/metrics", api_address);
    // println!("Proxy metrics available at: http://{}/metrics/proxy", api_address);

    let mut proxy = Proxy::new();

    // Add some example toxics
    proxy.add_toxic(Arc::new(LatencyToxic {
        latency: Duration::from_millis(100),
    }));
    proxy.add_toxic(Arc::new(CorruptToxic { probability: 0.01 }));
    proxy.add_toxic(Arc::new(SlowCloseToxic {
        delay: Duration::from_secs(1),
    }));

    // Start the proxy
    proxy.start(proxy_address.as_str(), upstream_address.as_str())
        .expect("Unable to start proxy");

    Ok(())
}
