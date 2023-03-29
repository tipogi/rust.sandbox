use std::{sync::Arc, collections::HashMap, io};
use mio::net::{TcpListener, TcpStream};

use crate::{open_connection::OpenConnection, server_mode::ServerMode};

// Token for our listening socket.
pub const LISTENER: mio::Token = mio::Token(0);

/// This binds together a TCP listening socket, some outstanding
/// connections, and a TLS server configuration.
pub struct TlsServer {
    server: TcpListener,
    connections: HashMap<mio::Token, OpenConnection>,
    next_id: usize,
    tls_config: Arc<rustls::ServerConfig>,
    mode: ServerMode,
}

impl TlsServer {
    pub fn new(server: TcpListener, mode: ServerMode, cfg: Arc<rustls::ServerConfig>) -> Self {
        Self {
            server,
            connections: HashMap::new(),
            next_id: 2,
            tls_config: cfg,
            mode,
        }
    }
    pub fn accept(&mut self, registry: &mio::Registry) -> Result<(), io::Error> {
        loop {
            match self.server.accept() {
                Ok((socket, addr)) => {
                    //debug!("Accepting new connection from {:?}", addr);
                    println!("Accepting new connection from {:?}", addr);

                    let tls_conn =
                        rustls::ServerConnection::new(Arc::clone(&self.tls_config)).unwrap();
                    let mode = self.mode.clone();

                    let token = mio::Token(self.next_id);
                    self.next_id += 1;
                    
                    println!("Openning connection with the client...");

                    let mut connection = OpenConnection::new(socket, token, mode, tls_conn);
                    connection.register(registry);
                    self.connections
                        .insert(token, connection);
                }
                Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => return Ok(()),
                Err(err) => {
                    println!(
                        "encountered error while accepting connection; err={:?}",
                        err
                    );
                    return Err(err);
                }
            }
        }
    }

    pub fn conn_event(&mut self, registry: &mio::Registry, event: &mio::event::Event) {
        let token = event.token();
        println!("Connection event! {:?}", token);

        if self.connections.contains_key(&token) {
            self.connections
                .get_mut(&token)
                .unwrap()
                .ready(registry, event);

            if self.connections[&token].is_closed() {
                self.connections.remove(&token);
            }
        }
    }
}