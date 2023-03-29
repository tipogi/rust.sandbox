use std::{fs, io::BufReader, sync::Arc};
use rustls::{self, server::NoClientAuth};
use crate::{args::Args};

fn load_certs(certs_path: &str) -> Vec<rustls::Certificate> {
    println!("Get the certificate content in '{}'", certs_path);
    let cert_file = fs::File::open(certs_path).expect("ERROR: Cannot open the certificates file");
    let mut reader = BufReader::new(cert_file);
    let certificates: Vec<rustls::Certificate> = rustls_pemfile::certs(&mut reader)
        .expect("The certificates are not well generated, try with new ones")
        .iter()
        .map(|v| rustls::Certificate(v.clone()))
        .collect();

    if certificates.len() == 0 {
        panic!("The files does not contain any certificates")
    }
    certificates
}

fn load_private_key(file_path: &str) -> rustls::PrivateKey {
    let key_buffer = fs::File::open(file_path).expect("ERROR: The private key could not find");
    let mut reader = BufReader::new(key_buffer);
    
    loop {
        match rustls_pemfile::read_one(&mut reader).expect("EERROR: Cannot parse private key .pem file") {
            Some(rustls_pemfile::Item::ECKey(key)) => return rustls::PrivateKey(key),
            Some(rustls_pemfile::Item::PKCS8Key(key)) => return rustls::PrivateKey(key),
            Some(rustls_pemfile::Item::RSAKey(key)) => return rustls::PrivateKey(key),
            _ => break,
        }
    }

    panic!(
        "ERROR: No keys found in {:?} (encrypted keys not supported)",
        file_path
    );
}

fn lookup_input_cipher_suites(suites: &[String]) -> Vec<rustls::SupportedCipherSuite> {
    let mut out = Vec::new();
    for cipher_suite_name in suites {
        let scs = find_cipher_suite(cipher_suite_name);
        match scs {
            Some(s) => out.push(s),
            None => panic!("ERROR: We cannot find the cipher suite {}", cipher_suite_name)
        }
    }
    out
}

fn find_cipher_suite(cipher_suite: &String) -> Option<rustls::SupportedCipherSuite> {
    for standard_cipher_suite in rustls::ALL_CIPHER_SUITES {
        let name = format!("{:?}", standard_cipher_suite.suite()).to_lowercase();
        if cipher_suite.to_string().to_lowercase() == name {
            return Some(*standard_cipher_suite)
        }
    }
    None
}

fn lookup_protocol_versions(versions: &Vec<String>) -> Vec<&'static rustls::SupportedProtocolVersion>{
    let mut out = Vec::new();
    for version_name in versions.iter() {
        let version = match version_name.as_ref() {
            "1.2" => &rustls::version::TLS12,
            "1.3" => &rustls::version::TLS12,
            _ => panic!("ERROR: We could not find common protocol versions")
        };
        out.push(version);
    }
    out
}

pub fn make_config(args: &Args) -> Arc<rustls::ServerConfig> {
    // Set default configurations
    let _client_auth = NoClientAuth::new();
    // online_certificate_status_protocol
    let ocsp = Vec::<u8>::new();

    // Extract the information from the command line parser
    let certs = load_certs(
        args.certs.as_ref()
    );
    let priv_key = load_private_key(
        args.key.as_ref()
    );

    let suites = if !args.suite.is_empty() {
        lookup_input_cipher_suites(&args.suite)
    } else {
        rustls::ALL_CIPHER_SUITES.to_vec()
    };

    let protocol_versions = if !args.prot_ver.is_empty() {
        lookup_protocol_versions(&args.prot_ver)
    } else {
        rustls::ALL_VERSIONS.to_vec()
    };

    let mut config = rustls::ServerConfig::builder()
        .with_cipher_suites(&suites)
        .with_safe_default_kx_groups()
        .with_protocol_versions(&protocol_versions)
        .expect("ERROR: Inconsistent cipher-suites/versions specified")
        .with_no_client_auth()
        // OCSP: Online certificate status protocol and SCT: Signed certificate timestamp
        .with_single_cert_with_ocsp_and_sct(certs, priv_key, ocsp, vec![])
        .expect("ERROR: Bad certificates/private key");
    
    println!("Created TLS server configuration");

    config.key_log = Arc::new(rustls::KeyLogFile::new());

    Arc::new(config)


}