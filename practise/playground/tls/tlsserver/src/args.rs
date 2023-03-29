use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// TLS server host name
   #[arg(long, default_value_t = String::from("127.0.0.1"))]
   pub host: String,

   /// TLS server listening port
   #[arg(short, long, value_name = "LOCAL_PORT", default_value_t = 8080)]
   pub port: u16,

   /// Read server certificates from CERTFILE.
   /// This should contain PEM-format certificates
   /// in the right order (the first certificate should
   /// certify KEYFILE, the last should be a root CA).
   #[arg(short, long, value_name = "CERTFILE")]
   pub certs: String,

   /// Read private key from KEYFILE. This should be a RSA
   /// private key or PKCS8-encoded private key, in PEM format.
   #[arg(short, long, value_name = "KEYFILE")]
   pub key: String,

   /// Disable default cipher suite list, and use
   // SUITE instead
   #[arg(long, num_args = 1.., value_delimiter = ' ', value_name = "CIPHER_SUITE(S)", required = false)]
   // To add that argument, we use commas
   pub suite: Vec<String>,

   /// Disable default TLS version list, and use
   /// the selected one instead. Example: 1.3
   #[arg(long, num_args = 1..2, value_delimiter = ' ', required = false)]
   pub prot_ver: Vec<String>,

   /// Server mode, establish the relationship with the client
   #[arg(long, default_value_t = String::from("http"))]
   pub mode: String,

   /// Read private key from KEYFILE.  This should be a RSA
   /// private key or PKCS8-encoded private key, in PEM format.
   #[arg(short, long, value_name = "BOOL", default_value_t = false)]
   require_auth: bool,

   ///Emit log output
   #[arg(long, value_name = "BOOL", default_value_t = false)]
   pub verbose: bool,
}