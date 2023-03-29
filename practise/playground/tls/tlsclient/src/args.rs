use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// TLS server host name
   #[arg(long, default_value_t = String::from("localhost"))]
   pub host: String,

   /// TLS server listening port
   #[arg(short, long, value_name = "LOCAL_PORT", default_value_t = 8080)]
   pub port: u16,

   /// Read root certificates from CAFILE
   #[arg(long, value_name = "CAFILE", required = false)]
   pub cafile: Option<String>,

   /// Disable default cipher suite list, and use
   // SUITE instead
   #[arg(long, num_args = 1.., value_delimiter = ' ', value_name = "CIPHER_SUITE(S)", required = false)]
   // To add that argument, we use commas
   pub suites: Vec<String>,

   /// Disable default TLS version list, and use
   /// the selected one instead. Example: 1.3
   #[arg(long, num_args = 1..2, value_delimiter = ' ', required = false)]
   pub prot_ver: Vec<String>,

   /// Disable certificate verification
   #[arg(long, value_name = "BOOL", default_value_t = false)]
   pub insecure: bool,

   /// Disable server name identification
   #[arg(long, value_name = "BOOL", default_value_t = true)]
   pub no_sni: bool,

   /// Emit log output
   #[arg(short, long, value_name = "BOOL", default_value_t = false)]
   pub verbose: bool,

   /// Send a basic HTTP GET request for /
   #[arg(long, value_name = "BOOL", default_value_t = false)]
   pub http: bool,

   /// Send ALPN extension containing PROTOCOL
   /// May be used multiple times to offer several protocols
   /// --proto http/1.1 (cannot find more compatible versions)
   #[arg(long, num_args = 1.., value_delimiter = ' ', value_name = "ALPN", required = false)]
   // To add that argument, we use commas
   pub proto: Vec<String>,
}