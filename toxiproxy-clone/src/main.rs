mod proxy;
mod toxic;
mod toxics;

use crate::proxy::Proxy;
use crate::toxics::corrupt::CorruptToxic;
use crate::toxics::latency::LatencyToxic;
use crate::toxics::slow_close::SlowCloseToxic;
use std::{
    io::{self},
    sync::Arc,
    time::Duration,
};


fn main() -> io::Result<()> {
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
    proxy.start("127.0.0.1:8474", "127.0.0.1:8475")
}
