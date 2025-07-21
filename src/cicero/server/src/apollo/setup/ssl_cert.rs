

use cicero::preludes::*;
use openssl::asn1::Asn1Time;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use openssl::x509::{X509, X509Builder, X509NameBuilder};
use std::fs::File;
use std::io::Write;
use crate::Error;

/// Generate self signed SSL cert
pub fn generate(datadir: &str) -> Result<(), Error> {
    let common_name = "localhost";
    let days_valid: u32 = 18520;

    // Generate RSA key pair (2048 bits is a good default)
    let rsa = Rsa::generate(2048)
        .map_err(|e| Error::OpenSSL(e.to_string()) )?;
    let pkey = PKey::from_rsa(rsa)
        .map_err(|e| Error::OpenSSL(e.to_string()) )?;

    // Create a new X509 certificate builder
    let mut x509_builder = X509Builder::new()
        .map_err(|e| Error::OpenSSL(e.to_string()) )?;

    // Set certificate version
    x509_builder.set_version(2)
        .map_err(|e| Error::OpenSSL(e.to_string()) )?;

    // Set random serial number
    let serial = openssl::bn::BigNum::from_u32(rand::random::<u32>())?.to_asn1_integer()
        .map_err(|e| Error::OpenSSL(e.to_string()) )?;
    x509_builder.set_serial_number(&serial)
        .map_err(|e| Error::OpenSSL(e.to_string()) )?;

    // Set validity period
    let not_before = Asn1Time::days_from_now(0)
        .map_err(|e| Error::OpenSSL(e.to_string()) )?;
    let not_after = Asn1Time::days_from_now(days_valid)
        .map_err(|e| Error::OpenSSL(e.to_string()) )?;
    x509_builder.set_not_before(&not_before)
        .map_err(|e| Error::OpenSSL(e.to_string()) )?;
    x509_builder.set_not_after(&not_after)
        .map_err(|e| Error::OpenSSL(e.to_string()) )?;

    // Set subject name
    let mut name_builder = X509NameBuilder::new()
        .map_err(|e| Error::OpenSSL(e.to_string()) )?;
    name_builder.append_entry_by_text("CN", common_name)
        .map_err(|e| Error::OpenSSL(e.to_string()) )?;
    let name = name_builder.build();

    x509_builder.set_subject_name(&name)
        .map_err(|e| Error::OpenSSL(e.to_string()) )?;
    x509_builder.set_issuer_name(&name)
        .map_err(|e| Error::OpenSSL(e.to_string()) )?;

    // Set public key
    x509_builder.set_pubkey(&pkey)
        .map_err(|e| Error::OpenSSL(e.to_string()) )?;

    // Sign the certificate with our private key
    x509_builder.sign(&pkey, MessageDigest::sha256())
        .map_err(|e| Error::OpenSSL(e.to_string()) )?;

    // Get the final certificate
    let cert = x509_builder.build();

    File::create(format!("{}/server/ssl/cicero.pem", datadir))?.write_all(&cert.to_pem()?)?;
    File::create(format!("{}/server/ssl/privkey.pem", datadir))?.write_all(&pkey.private_key_to_pem_pkcs8()?)?;

    Ok(())
}


