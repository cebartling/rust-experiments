use crate::toxic::Toxic;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::{io, thread};

pub struct Proxy {
    pub toxics: Vec<Arc<dyn Toxic>>,
}

impl Proxy {
    pub fn new() -> Self {
        Proxy { toxics: Vec::new() }
    }

    pub fn add_toxic(&mut self, toxic: Arc<dyn Toxic>) {
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

    pub fn start(&self, listen_addr: &str, upstream_addr: &str) -> io::Result<()> {
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
