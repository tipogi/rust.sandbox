use std::{sync::Arc, fs, io::BufReader};
use rustls::{self, RootCertStore, OwnedTrustAnchor};
use rustls_pemfile;

use crate::args::Args;

#[cfg(not(feature = "dangerous_configuration"))]
mod danger {
    pub struct NoCertificateVerification {}

    impl rustls::client::ServerCertVerifier for NoCertificateVerification {
        fn verify_server_cert(
            &self,
            _end_entity: &rustls::Certificate,
            _intermediates: &[rustls::Certificate],
            _server_name: &rustls::ServerName,
            _scts: &mut dyn Iterator<Item = &[u8]>,
            _ocsp: &[u8],
            _now: std::time::SystemTime,
        ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
            println!("\n[#cfg-dangerous_configuration] No server verification\n");
            Ok(rustls::client::ServerCertVerified::assertion())
        }
    }
}

#[cfg(not(feature = "dangerous_configuration"))]
fn apply_dangerous_options(args: &Args, cfg: &mut rustls::ClientConfig) {
    if args.insecure {
        println!("Dangerous configuration, just for development. Not set server certificate verification");
        cfg.dangerous()
            .set_certificate_verifier(Arc::new(danger::NoCertificateVerification {}));
    }
}

#[cfg(feature = "dangerous_configuration")]
fn apply_dangerous_options(args: &Args, _: &mut rustls::ClientConfig) {
    if args.insecure {
        panic!("This build does not support --insecure.");
    }
}

pub fn make_config(args: &Args) -> Arc<rustls::ClientConfig> {
    let root_store = create_root_store(&args);
    let suites = if !args.suites.is_empty() {
        lookup_suites(&args.suites)
    } else {
        rustls::DEFAULT_CIPHER_SUITES.to_vec()
    };

    let version = if !args.prot_ver.is_empty() {
        lookup_versions(&args.prot_ver)
    } else {
        rustls::DEFAULT_VERSIONS.to_vec()
    };

    let config = rustls::ClientConfig::builder()
        .with_cipher_suites(&suites)
        .with_safe_default_kx_groups()
        .with_protocol_versions(&version)
        .expect("ERROR: Inconsistent cipher-suite/version selected")
        .with_root_certificates(root_store);
    
    let mut config = config.with_no_client_auth();

    if args.no_sni {
        config.enable_sni = false;
    }

    config.alpn_protocols = args
        .proto
        .iter()
        .map(|proto| {
            println!("ALPN: {:?}", proto);
            proto.as_bytes().to_vec()
        })
        .collect();

    apply_dangerous_options(args, &mut config);

    Arc::new(config)
    
}


fn lookup_versions(versions: &Vec<String>) -> Vec<&'static rustls::SupportedProtocolVersion>{
    let mut new_versions = Vec::new();

    for version in versions.iter() {
        match version.as_str() {
            "1.2" => new_versions.push(&rustls::version::TLS12),
            "1.3" => new_versions.push(&rustls::version::TLS13),
            _ => panic!("ERROR: The selected version is not available in the client")
        }
    }
    new_versions
}

fn find_suite(cipher_suite: &String) -> Option<rustls::SupportedCipherSuite> {
    for default_suite in rustls::ALL_CIPHER_SUITES {
        let suite = format!("{:?}", default_suite.suite()).to_lowercase();
        if suite == cipher_suite.to_lowercase() {
            return Some(*default_suite)
        }
    }
    None
}

fn lookup_suites(suites: &Vec<String>) -> Vec<rustls::SupportedCipherSuite>{
    let mut new_suites = Vec::new();

    for custom_suite in suites.iter() {
        let supported_cipher_suite = find_suite(custom_suite);
        match supported_cipher_suite {
            Some(s) => new_suites.push(s),
            None => panic!("ERROR: Cannot lookup cipher suite {:?}", custom_suite)
        }
    }
    new_suites
}

/// Create client root certificate store to check the validity of the server
/// certificate if it was signed for an authorized certificate authority
fn create_root_store(args: &Args) -> RootCertStore {
    let mut root_store = RootCertStore::empty();

    if args.cafile.is_some() {
        let ca_file = args.cafile.as_ref().unwrap();
        let certificate = fs::File::open(ca_file).expect("ERROR: Cannot find CA file");
        let mut reader = BufReader::new(certificate);
        root_store.add_parsable_certificates(&rustls_pemfile::certs(&mut reader).unwrap());
    } else {
        root_store.add_server_trust_anchors(
            webpki_roots::TLS_SERVER_ROOTS
                .0
                .iter()
                .map(|ta| {
                    OwnedTrustAnchor::from_subject_spki_name_constraints(
                        ta.subject,
                        ta.spki,
                        ta.name_constraints
                    )
                })
        );
    }
    root_store
}