use std::env;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use std::sync::Arc;
use deadpool_postgres::{Config, ManagerConfig, Pool, PoolConfig, RecyclingMethod, Runtime, Timeouts};
use base64::Engine;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::pem::PemObject;
use rustls::RootCertStore;
use tokio_postgres::NoTls;
use tokio_postgres_rustls::MakeRustlsConnect;

pub fn build_pool(key: &str) -> Arc<Pool> {
    let mut config = Config::new();
    match env::var(key) {
        Ok(db_url) => config.url = Some(db_url),
        _ => {
            println!("Environment variable {key} could not be read");
            exit(1);
        }
    }
    let mut pool_config = PoolConfig::new(75);
    pool_config.timeouts = Timeouts::wait_millis(10000);
    config.pool = Some(pool_config);
    config.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });
    let cert_file_key = format!("{}_CERT_FILE", key);
    if let Ok(cert_file) = env::var(cert_file_key.as_str()) {
        let mut store = RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.into(),
        };
        let path_buf = PathBuf::from_str(cert_file.as_str()).unwrap();
        let path = path_buf.as_path();
        let cert = CertificateDer::from_pem_file(path).unwrap();
        store.add(cert).unwrap();
        let tls_config = rustls::ClientConfig::builder()
            .with_root_certificates(store)
            .with_no_client_auth();
        let tls = MakeRustlsConnect::new(tls_config);
        return Arc::new(config.create_pool(Some(Runtime::Tokio1), tls).unwrap());
    }
    let cert_b64_key = format!("{}_CERT_B64", key);
    if let Ok(cert) = env::var(cert_b64_key.as_str()) {
        let mut store = RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.into(),
        };
        let bytes = cert.into_bytes();
        let decoded = base64::prelude::BASE64_STANDARD.decode(bytes).unwrap();
        let cert = CertificateDer::from_pem_slice(&decoded).unwrap();
        store.add(cert).unwrap();
        let tls_config = rustls::ClientConfig::builder()
            .with_root_certificates(store)
            .with_no_client_auth();
        let tls = MakeRustlsConnect::new(tls_config);
        return Arc::new(config.create_pool(Some(Runtime::Tokio1), tls).unwrap());
    }
    Arc::new(config.create_pool(Some(Runtime::Tokio1), NoTls).unwrap())
}