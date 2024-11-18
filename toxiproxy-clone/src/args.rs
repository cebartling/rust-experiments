use clap::Parser;

// CLI Arguments
#[derive(Parser, Debug)]
#[command(
    name = "toxiproxy",
    about = "A TCP proxy for simulating network conditions",
    version
)]
pub struct Args {
    /// Port for the REST API
    #[arg(long, default_value = "8474")]
    pub api_port: u16,

    /// Port the proxy listens on
    #[arg(long, default_value = "8475")]
    pub proxy_port: u16,

    /// Port of the upstream service
    #[arg(long, default_value = "8476")]
    pub upstream_port: u16,

    /// Host address to bind to
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    /// Upstream host address
    #[arg(long, default_value = "127.0.0.1")]
    pub upstream_host: String,
}
