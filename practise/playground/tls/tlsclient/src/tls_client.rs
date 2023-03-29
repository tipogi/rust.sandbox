use std::sync::Arc;
use std::process;
use std::io;
use std::io::{Read, Write};

use std::str;

use mio::net::TcpStream;

const CLIENT: mio::Token = mio::Token(0);

/// This encapsulates the TCP-level connection, some connection
/// state, and the underlying TLS-level session.
pub struct TlsClient {
    socket: TcpStream,
    closing: bool,
    clean_closure: bool,
    tls_conn: rustls::ClientConnection,
}

impl TlsClient {
    pub fn new(
        sock: TcpStream,
        server_name: rustls::ServerName,
        cfg: Arc<rustls::ClientConfig>,
    ) -> Self {
        Self {
            socket: sock,
            closing: false,
            clean_closure: false,
            tls_conn: rustls::ClientConnection::new(cfg, server_name).unwrap(),
        }
    }

    /// Handles events sent to the TlsClient by mio::Poll
    pub fn ready(&mut self, ev: &mio::event::Event) {
        println!("[TLS_Client, tls_client.rs] Ready! with event token of {:?}", ev.token());
        //println!("Ready! with event token of {:#?}", ev);
        assert_eq!(ev.token(), CLIENT);

        if ev.is_readable() {
            println!("[TLS_Client, tls_client.rs] Is readable event");
            self.do_read();
        }
        
        if ev.is_writable() {
            println!("[TLS_Client, tls_client.rs] Is writable event");
            self.do_write();
        }
        
        if self.is_closed() {
            println!("[TLS_Client, tls_client.rs] Connection closed, clean_clousure: {:?}", self.clean_closure);
            process::exit(if self.clean_closure { 0 } else { 1 });
        }
    }

    pub fn read_source_to_end(&mut self, rd: &mut dyn io::Read) -> io::Result<usize> {
        println!("[TLS_Client, tls_client.rs] Read source to end...");
        let mut buf = Vec::new();
        println!("[TLS_Client, tls_client.rs] Create new buffer");
        let len = rd.read_to_end(&mut buf)?;
        println!("[TLS_Client, tls_client.rs] Length: {:?}", len);
        self.tls_conn
            .writer()
            .write_all(&buf)
            .unwrap();
        Ok(len)
    }

    /// We're ready to do a read.
    pub fn do_read(&mut self) {
        // Read TLS data.  
        // This fails if the underlying TCP connection is broken.
        match self.tls_conn.read_tls(&mut self.socket) {
            Err(error) => {
                println!("[TLS_Client, tls_client.rs] do_read() ERROR: {:?}", error);
                if error.kind() == io::ErrorKind::WouldBlock {
                    return;
                }
                self.closing = true;
                return;
            }

            // If we're ready but there's no data: EOF.
            Ok(0) => {
                println!("[TLS_Client, tls_client.rs] We're ready but there's no data: EOF");
                self.closing = true;
                self.clean_closure = true;
                return;
            }

            Ok(ok) => {
                println!("[TLS_Client, tls_client.rs] Succesfull read: {:?}", ok);
            }
        };

        // Reading some TLS data might have yielded new TLS
        // messages to process. Errors from this indicate
        // TLS protocol problems and are fatal.
        let io_state = match self.tls_conn.process_new_packets() {
            Ok(io_state) => io_state,
            Err(err) => {
                println!("[TLS_Client, tls_client.rs] TLS error processing new packets: {:?}", err);
                self.closing = true;
                return;
            }
        };

        // Having read some TLS data, and processed any new messages,
        // we might have new plaintext as a result.
        //
        // Read it and then write it to stdout.
        if io_state.plaintext_bytes_to_read() > 0 {
            let mut plaintext = Vec::new();
            plaintext.resize(io_state.plaintext_bytes_to_read(), 0u8);
            self.tls_conn
                .reader()
                .read_exact(&mut plaintext)
                .unwrap();
            io::stdout()
                .write_all(&plaintext)
                .unwrap();
        }

        // If wethat fails, the peer might have started a clean TLS-level
        // session closure.
        if io_state.peer_has_closed() {
            self.clean_closure = true;
            self.closing = true;
        }
    }

    pub fn do_write(&mut self) {
        self.tls_conn
            .write_tls(&mut self.socket)
            .unwrap();
    }

    /// Registers self as a 'listener' in mio::Registry
    pub fn register(&mut self, registry: &mio::Registry) {
        println!("[TLS_Client, tls_client.rs] Register the listener in mio::Registry");
        let interest = self.event_set();
        registry
            .register(&mut self.socket, CLIENT, interest)
            .unwrap();
    }

    /// Reregisters self as a 'listener' in mio::Registry.
    pub fn reregister(&mut self, registry: &mio::Registry) {
        let interest = self.event_set();
        registry
            .reregister(&mut self.socket, CLIENT, interest)
            .unwrap();
    }

    /// Use wants_read/wants_write to register for different mio-level
    /// IO readiness events.
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
        self.closing
    }
}
impl io::Write for TlsClient {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        println!("[TLS_Client, tls_client.rs] Write in the stream");
        self.tls_conn.writer().write(bytes)
    }
    
    fn flush(&mut self) -> io::Result<()> {
        self.tls_conn.writer().flush()
    }
}

impl io::Read for TlsClient {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        println!("[TLS_Client, tls_client.rs] Read from the stream...");
        println!("{:?}", str::from_utf8(bytes));
        self.tls_conn.reader().read(bytes)
    }
}