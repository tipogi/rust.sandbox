use core::task::{Context, Poll};
use futures_util::ready;

use hyper::server::conn::{AddrIncoming, AddrStream};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::server::accept::Accept;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::vec::Vec;
use std::{io, env, fs, sync};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio_rustls::rustls::ServerConfig;

fn main() {
    if let Err(e) = run_server() {
        eprintln!("Failed: {}", e);
        std::process::exit(1);
    }
}


fn error(err: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}

#[tokio::main]
async fn run_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    let port = match env::args().nth(1) {
        Some(ref p) => p.to_owned(),
        None => "9009".to_owned()
    };

    let address = format!("127.0.0.1:{}", port).parse()?;

    //Build TLS configuration
    let tls_config = {
        // Load public certificates
        let certs = load_certs("certificates/rusty.pem")?;
        // Load private key
        let key = load_private_key("certificates/rusty.rsa")?;

        let mut cfg = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|e| error(format!("{}", e)))?;

        cfg.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec(), b"http/1.0".to_vec()];
        sync::Arc::new(cfg)
    };

    let incoming = hyper::server::conn::AddrIncoming::bind(&address)?;
    // Create the routes of the endpoints in the server
    let service = make_service_fn(|_| async { Ok::<_, io::Error>(service_fn(routes)) });
    let server = Server::builder(TlsAcceptor::new(tls_config, incoming)).serve(service);

    // Run the future, keep going until an error occurs.
    println!("Starting to serve on https://{}.", address);
    server.await?;
    
    Ok(())
}

enum State {
    Handshaking(tokio_rustls::Accept<AddrStream>),
    Streaming(tokio_rustls::server::TlsStream<AddrStream>),
}

// tokio_rustls::server::TlsStream doesn't expose constructor methods,
// so we have to TlsAcceptor::accept and handshake to have access to it
// TlsStream implements AsyncRead/AsyncWrite handshaking tokio_rustls::Accept first
pub struct TlsStream {
    state: State,
}

impl TlsStream {
    fn new(stream: AddrStream, config: Arc<ServerConfig>) -> Self {
        let accept = tokio_rustls::TlsAcceptor::from(config).accept(stream);
        Self {
            state: State::Handshaking(accept)
        }
    }
}

impl AsyncRead for TlsStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut ReadBuf
    ) -> Poll<io::Result<()>> {
        let pin = self.get_mut();
        match pin.state {
            State::Handshaking(ref mut accept) => match ready!(Pin::new(accept).poll(cx)) {
                Ok(mut stream) => {
                    let result = Pin::new(&mut stream).poll_read(cx, buf);
                    pin.state = State::Streaming(stream);
                    result
                }
                Err(err) => Poll::Ready(Err(err))
            },
            State::Streaming(ref mut stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for TlsStream {
    fn poll_write(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<Result<usize, io::Error>> {
        let pin = self.get_mut();

        match pin.state {
            State::Handshaking(ref mut accept) => match ready!(Pin::new(accept).poll(cx)) {
                Ok(mut stream) => {
                    let result = Pin::new(&mut stream).poll_write(cx, buf);
                    pin.state = State::Streaming(stream);
                    result
                }
                Err(err) => Poll::Ready(Err(err))
            },
            State::Streaming(ref mut stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.state {
            State::Handshaking(_) => Poll::Ready(Ok(())),
            State::Streaming(ref mut stream) => Pin::new(stream).poll_flush(cx),
        }
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.state {
            State::Handshaking(_) => Poll::Ready(Ok(())),
            State::Streaming(ref mut stream) => Pin::new(stream).poll_shutdown(cx),
        }
    }
}

pub struct TlsAcceptor {
    config: Arc<ServerConfig>,
    incoming: AddrIncoming
}

impl TlsAcceptor {
    pub fn new(config: Arc<ServerConfig>, incoming: AddrIncoming) -> Self {
        Self {
            config,
            incoming
        }
    }
}

impl Accept for TlsAcceptor {
    type Conn = TlsStream;
    type Error = io::Error;

    fn poll_accept(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> std::task::Poll<Option<Result<Self::Conn, Self::Error>>> {
        let pin = self.get_mut();
        match ready!(Pin::new(&mut pin.incoming).poll_accept(cx)) {
            Some(Ok(sock)) => Poll::Ready(Some(Ok(TlsStream::new(sock, pin.config.clone())))),
            Some(Err(e)) => Poll::Ready(Some(Err(e))),
            None => Poll::Ready(None)
        }
    }

}

///////////////////////////////////////
///////// CREATE ECHO SERVER //////////
///////////////////////////////////////
// Custom echo service, handling two different routes and a
// catch-all 404 responder.
async fn routes(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());
    match(req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POST /echo\n")
        }// Echo service route.
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        }
        // Catch-all 404.
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };
    Ok(response)
}


///////////////////////////////////////
///// LOAD SERVER CERTIFICATES ////////
///////////////////////////////////////

fn load_certs(filename: &str) -> io::Result<Vec<rustls::Certificate>> {
    let cert_file = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;

    let mut reader = io::BufReader::new(cert_file);

    // Load an return the certificate
    let certificates = rustls_pemfile::certs(&mut reader)
        .map_err(|_| error("The certificates are not well generated, try with new ones".into()))?;

    Ok(certificates
        .into_iter()
        .map(rustls::Certificate)
        .collect()
    )
}

fn load_private_key(private_key_file: &str) -> io::Result<rustls::PrivateKey> {
    let key_file = fs::File::open(private_key_file)
        .map_err(|e| error(format!("Could not find the file {}:{}", private_key_file, e)))?;

    let mut reader = io::BufReader::new(key_file);

    loop {
        match rustls_pemfile::read_one(&mut reader).expect("EERROR: Cannot parse private key .pem file") {
            Some(rustls_pemfile::Item::ECKey(key)) => return Ok(rustls::PrivateKey(key)),
            Some(rustls_pemfile::Item::PKCS8Key(key)) => return Ok(rustls::PrivateKey(key)),
            Some(rustls_pemfile::Item::RSAKey(key)) => return Ok(rustls::PrivateKey(key)),
            _ => break,
        }
    }

    panic!(
        "ERROR: No keys found in {:?} (encrypted keys not supported)",
        private_key_file
    );
}

