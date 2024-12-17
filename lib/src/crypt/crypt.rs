//use rustls::quic::
use std::io;
use rustls::{
    Connection,
    Error,
    // ConnectionCommon,
    RootCertStore,
    Stream,
    Tls13CipherSuite,
};
use sqlx::{
    Pool,
    MySql,
};
use x509_parser::{
    prelude::*,
};
//use std::sync::Arc;

struct certVerifier {
    rCertStore: RootCertStore,
}

impl certVerifier {
    pub fn certStoreSetup() -> Result<Self> {
        let mut rCertStore = RootCertStore::empty();
        
    }
}