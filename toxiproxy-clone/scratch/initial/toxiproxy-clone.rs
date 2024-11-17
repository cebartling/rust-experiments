use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
    time::Duration,
};
use rand::Rng;

// Toxic trait defines behavior modifications for the proxy
trait Toxic: Send + Sync {
    fn modify_upstream(&self, data: &mut Vec<u8>);
    fn modify_downstream(&self, data: &mut Vec<u8>);
}

// Latency toxic adds delay to the connection
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
}

// SlowCloseToxic delays connection closing
struct SlowCloseToxic {
    delay: Duration,
}

impl Toxic for SlowCloseToxic {
    fn modify_upstream(&self, _data: &mut Vec<u8>) {}
    fn modify_downstream(&self, _data: &mut Vec<u8>) {}
}

// Random data corruption toxic
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
}

struct Proxy {
    toxics: Vec<Arc<dyn Toxic>>,
}

impl Proxy {
    fn new() -> Self {
        Proxy { toxics: Vec::new() }
    }

    fn add_toxic(&mut self, toxic: Arc<dyn Toxic>) {
        self.toxics.push(toxic);
    }

    fn handle_connection(
        upstream: TcpStream,
        downstream_addr: String,
        toxics: Arc<Vec<Arc<dyn Toxic>>>,
    ) -> io::Result<()> {
        let downstream = TcpStream::connect(downstream_addr)?;
        let upstream_clone = upstream.try_clone()?;
        let downstream_clone = downstream.try_clone()?;

        // Handle upstream -> downstream
        let toxics_clone = Arc::clone(&toxics);
        thread::spawn(move || {
            Self::proxy_data(upstream, downstream, toxics_clone, true);
        });

        // Handle downstream -> upstream
        let toxics_clone = Arc::clone(&toxics);
        thread::spawn(move || {
            Self::proxy_data(downstream_clone, upstream_clone, toxics_clone, false);
        });

        Ok(())
    }

    fn proxy_data(
        mut from: TcpStream,
        mut to: TcpStream,
        toxics: Arc<Vec<Arc<dyn Toxic>>>,
        is_upstream: bool,
    ) {
        let mut buffer = vec![0; 4096];
        loop {
            match from.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    let mut data = buffer[..n].to_vec();
                    
                    // Apply toxics
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

    fn start(&self, listen_addr: &str, upstream_addr: &str) -> io::Result<()> {
        let listener = TcpListener::bind(listen_addr)?;
        let toxics = Arc::new(self.toxics.clone());

        println!("Proxy listening on {}", listen_addr);
        println!("Forwarding to {}", upstream_addr);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let upstream_addr = upstream_addr.to_string();
                    let toxics = Arc::clone(&toxics);
                    thread::spawn(move || {
                        if let Err(e) = Self::handle_connection(stream, upstream_addr, toxics) {
                            eprintln!("Connection error: {}", e);
                        }
                    });
                }
                Err(e) => eprintln!("Connection failed: {}", e),
            }
        }

        Ok(())
    }
}

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
