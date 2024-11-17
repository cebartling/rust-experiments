# Rust Toxiproxy Clone

A Rust implementation of [Shopify's Toxiproxy](https://github.com/shopify/toxiproxy) - a TCP proxy designed for
simulating network conditions in testing environments. This tool helps make your application more resilient by
simulating various network failures and conditions.

## Features

- **TCP Proxying**: Forward TCP traffic between client and upstream services
- **Dynamic Configuration**: REST API for runtime toxic configuration
- **Metrics Collection**: Prometheus-compatible metrics and detailed proxy statistics
- **Multiple Toxic Types**:
    - Latency: Add delay to connections
    - Corrupt: Randomly corrupt TCP packets
    - SlowClose: Delay connection termination

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/rust-toxiproxy
cd rust-toxiproxy

# Build the project
cargo build --release

# Run the proxy
cargo run
```

## Usage

### Starting the Proxy

The proxy server starts with the following default configuration:

- REST API: `localhost:8474`
- Proxy listener: `localhost:8475`
- Upstream service: `localhost:8476`

```bash
cargo run
```

### REST API Endpoints

#### List All Toxics

```bash
curl http://localhost:8474/toxics
```

#### Add New Toxic

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

### Available Toxic Configurations

#### Latency Toxic

Adds delay to connections

```json
{
  "proxy": "main",
  "config": {
    "type": "Latency",
    "latency_ms": 100
  }
}
```

#### Corrupt Toxic

Randomly corrupts TCP packets

```json
{
  "proxy": "main",
  "config": {
    "type": "Corrupt",
    "probability": 0.01
  }
}
```

#### SlowClose Toxic

Delays connection termination

```json
{
  "proxy": "main",
  "config": {
    "type": "SlowClose",
    "delay_ms": 1000
  }
}
```

## Metrics

### Prometheus Metrics

Available at `GET /metrics`

Collected metrics include:

- `toxiproxy_bytes_transferred`: Total bytes transferred through proxy
- `toxiproxy_active_connections`: Number of active connections
- `toxiproxy_toxic_activations`: Number of times each toxic was activated
- `toxiproxy_latency_seconds`: Histogram of latency measurements
- `toxiproxy_corruptions`: Number of times data was corrupted

### Detailed Proxy Metrics

Available at `GET /metrics/proxy`

```bash
curl http://localhost:8474/metrics/proxy
```

Example response:

```json
{
  "main": {
    "bytes_upstream": 1234567,
    "bytes_downstream": 7654321,
    "active_connections": 5,
    "toxic_activations": {
      "latency": 100,
      "corrupt": 10,
      "slow_close": 5
    },
    "latency_percentiles": {
      "p50": 0.1,
      "p90": 0.2,
      "p95": 0.3,
      "p99": 0.5
    },
    "corruptions": 10
  }
}
```

## Monitoring

### Prometheus Configuration

Add the following to your `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'toxiproxy'
    static_configs:
      - targets: [ 'localhost:8474' ]
```

### Example Grafana Dashboard

You can create dashboards to monitor:

- Bytes transferred (upstream/downstream)
- Active connections
- Toxic activation counts
- Latency percentiles
- Corruption rates

## Development

### Project Structure

```
src/
├── main.rs          # Main application entry point
├── proxy.rs         # Proxy implementation
├── toxic/           # Toxic implementations
│   ├── mod.rs
│   ├── latency.rs
│   ├── corrupt.rs
│   └── slow_close.rs
├── api/             # REST API handlers
│   ├── mod.rs
│   └── metrics.rs
└── metrics/         # Metrics collection
    └── mod.rs
```

### Adding New Toxics

1. Create a new struct implementing the `Toxic` trait:

```rust
struct MyToxic {
    // toxic configuration
}

impl Toxic for MyToxic {
    fn modify_upstream(&self, data: &mut Vec<u8>, proxy_name: &str) {
        // implement toxic behavior
    }
    // ... implement other required methods
}
```

2. Add the toxic configuration to `ToxicConfig` enum
3. Update the toxic creation logic in the REST API handler

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Inspired by [Shopify's Toxiproxy](https://github.com/shopify/toxiproxy)
- Built with [Rust](https://www.rust-lang.org/)
