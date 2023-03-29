use std::{net, io::{self, Read, Write}, str};

use mio::net::TcpStream;
use crate::server_mode::ServerMode;

/// This is a connection which has been accepted by the server,
/// and is currently being served.
///
/// It has a TCP-level stream, a TLS-level connection state, and some
/// other state/metadata.
#[derive(Debug)]
pub struct OpenConnection {
    socket: TcpStream,
    token: mio::Token,
    closing: bool,
    closed: bool,
    mode: ServerMode,
    tls_conn: rustls::ServerConnection,
    back: Option<TcpStream>,
    sent_http_response: bool,
}

/// Open a plaintext TCP-level connection for forwarded connections.
fn open_back(mode: &ServerMode) -> Option<TcpStream> {
    match *mode {
        ServerMode::Forward(ref port) => {
            let addr = net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), *port);
            let conn = TcpStream::connect(net::SocketAddr::V4(addr)).unwrap();
            Some(conn)
        }
        _ => None,
    }
}

/// This used to be conveniently exposed by mio: map EWOULDBLOCK
/// errors to something less-errory.
fn try_read(r: io::Result<usize>) -> io::Result<Option<usize>> {
    match r {
        Ok(len) => Ok(Some(len)),
        Err(e) => {
            if e.kind() == io::ErrorKind::WouldBlock {
                Ok(None)
            } else {
                Err(e)
            }
        }
    }
}

impl OpenConnection {
    pub fn new(
        socket: TcpStream,
        token: mio::Token,
        mode: ServerMode,
        tls_conn: rustls::ServerConnection,
    ) -> Self {
        let back = open_back(&mode);
        Self {
            socket,
            token,
            closing: false,
            closed: false,
            mode,
            tls_conn,
            back,
            sent_http_response: false,
        }
    }

    /// We're a connection, and we have something to do.
    pub fn ready(&mut self, registry: &mio::Registry, ev: &mio::event::Event) {
        // If we're readable: read some TLS.  Then
        // see if that yielded new plaintext.  Then
        // see if the backend is readable too.
        if ev.is_readable() {
            self.do_tls_read();
            self.try_plain_read();
            self.try_back_read();
        }

        if ev.is_writable() {
            self.do_tls_write_and_handle_error();
        }

        if self.closing {
            let _ = self
                .socket
                .shutdown(net::Shutdown::Both);
            self.close_back();
            self.closed = true;
            self.deregister(registry);
        } else {
            self.reregister(registry);
        }
    }

    /// Close the backend connection for forwarded sessions.
    pub fn close_back(&mut self) {
        if self.back.is_some() {
            let back = self.back.as_mut().unwrap();
            back.shutdown(net::Shutdown::Both)
                .unwrap();
        }
        self.back = None;
    }

    pub fn do_tls_read(&mut self) {
        // Read some TLS data.
        match self.tls_conn.read_tls(&mut self.socket) {
            Err(err) => {
                if let io::ErrorKind::WouldBlock = err.kind() {
                    return;
                }

                //error!("read error {:?}", err);
                println!("read error {:?}", err);
                self.closing = true;
                return;
            }
            Ok(0) => {
                println!("eof");
                //debug!("eof");
                self.closing = true;
                return;
            }
            Ok(_) => {}
        };

        // Process newly-received TLS messages.
        if let Err(err) = self.tls_conn.process_new_packets() {
            //error!("cannot process packet: {:?}", err);
            println!("cannot process packet: {:?}", err);

            // last gasp write to send any alerts
            self.do_tls_write_and_handle_error();

            self.closing = true;
        }
    }

    pub fn try_plain_read(&mut self) {
        // Read and process all available plaintext.
        if let Ok(io_state) = self.tls_conn.process_new_packets() {
            if io_state.plaintext_bytes_to_read() > 0 {
                let mut buf = Vec::new();
                buf.resize(io_state.plaintext_bytes_to_read(), 0u8);

                self.tls_conn
                    .reader()
                    .read_exact(&mut buf)
                    .unwrap();

                //debug!("plaintext read {:?}", buf.len());
                println!("plaintext read {:?}", buf.len());
                self.incoming_plaintext(&buf);
            }
        }
    }

    pub fn try_back_read(&mut self) {
        if self.back.is_none() {
            return;
        }

        // Try a non-blocking read.
        let mut buf = [0u8; 1024];
        let back = self.back.as_mut().unwrap();
        let rc = try_read(back.read(&mut buf));

        if rc.is_err() {
            println!("backend read failed: {:?}", rc);
            //error!("backend read failed: {:?}", rc);
            self.closing = true;
            return;
        }

        let maybe_len = rc.unwrap();

        // If we have a successful but empty read, that's an EOF.
        // Otherwise, we shove the data into the TLS session.
        match maybe_len {
            Some(len) if len == 0 => {
                println!("back eof");
                //debug!("back eof");
                self.closing = true;
            }
            Some(len) => {
                self.tls_conn
                    .writer()
                    .write_all(&buf[..len])
                    .unwrap();
            }
            None => {}
        };
    }

    /// Process some amount of received plaintext.
    pub fn incoming_plaintext(&mut self, buf: &[u8]) {
        println!("CLient message: {:?}", str::from_utf8(buf).unwrap());
        println!("Incoming message mode: {:?}", self.mode);
        match self.mode {
            ServerMode::Echo => {
                self.tls_conn
                    .writer()
                    .write_all(buf)
                    .unwrap();
            }
            ServerMode::Http => {
                self.send_http_response_once();
            }
            ServerMode::Forward(_) => {
                self.back
                    .as_mut()
                    .unwrap()
                    .write_all(buf)
                    .unwrap();
            }
        }
    }

    pub fn send_http_response_once(&mut self) {
        let response =
            b"HTTP/1.0 200 OK\r\nConnection: close\r\n\r\nHello world from rustls tlsserver\r\n";
        if !self.sent_http_response {
            self.tls_conn
                .writer()
                .write_all(response)
                .unwrap();
            self.sent_http_response = true;
            self.tls_conn.send_close_notify();
        }
    }

    pub fn tls_write(&mut self) -> io::Result<usize> {
        self.tls_conn
            .write_tls(&mut self.socket)
    }

    pub fn do_tls_write_and_handle_error(&mut self) {
        let rc = self.tls_write();
        if rc.is_err() {
            println!("write failed {:?}", rc);
            //error!("write failed {:?}", rc);
            self.closing = true;
        }
    }

    pub fn register(&mut self, registry: &mio::Registry) {
        let event_set = self.event_set();
        registry
            .register(&mut self.socket, self.token, event_set)
            .unwrap();

        if self.back.is_some() {
            registry
                .register(
                    self.back.as_mut().unwrap(),
                    self.token,
                    mio::Interest::READABLE,
                )
                .unwrap();
        }
    }

    pub fn reregister(&mut self, registry: &mio::Registry) {
        let event_set = self.event_set();
        registry
            .reregister(&mut self.socket, self.token, event_set)
            .unwrap();
    }

    pub fn deregister(&mut self, registry: &mio::Registry) {
        registry
            .deregister(&mut self.socket)
            .unwrap();

        if self.back.is_some() {
            registry
                .deregister(self.back.as_mut().unwrap())
                .unwrap();
        }
    }

    /// What IO events we're currently waiting for,
    /// based on wants_read/wants_write.
    pub fn event_set(&self) -> mio::Interest {
        let rd = self.tls_conn.wants_read();
        let wr = self.tls_conn.wants_write();

        if rd && wr {
            mio::Interest::READABLE | mio::Interest::WRITABLE
        } else if wr {
            mio::Interest::WRITABLE
        } else {
            mio::Interest::READABLE
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }
}