use std::{str::FromStr, time::Duration};

use p256::{ecdsa::{signature::Verifier, Signature, SigningKey, VerifyingKey}, pkcs8::{DecodePublicKey, EncodePublicKey}};
use x509_cert::{builder::{Builder, CertificateBuilder, Profile}, der::{Decode, Encode}, name::Name, serial_number::SerialNumber, time::Validity, Certificate};

fn verify_certificate(der: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let cert = Certificate::from_der(der)?;
    assert_eq!(cert.tbs_certificate.subject, cert.tbs_certificate.issuer);
    let spki = cert.tbs_certificate.subject_public_key_info.to_der()?;
    let key = VerifyingKey::from_public_key_der(&spki)?;
    let signature = Signature::from_der(cert.signature.as_bytes().ok_or("unaligned signature")?)?;
    let tbs = cert.tbs_certificate.to_der()?;
    key.verify(&tbs, &signature)?;
    let mut changed = tbs;
    changed[0] ^= 1;
    assert!(key.verify(&changed, &signature).is_err());
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = rcgen::KeyPair::generate_for(&rcgen::PKCS_ECDSA_P256_SHA256)?;
    let params = rcgen::CertificateParams::new(vec!["kenaz.example".to_owned()])?;
    let cert = params.self_signed(&key)?;
    verify_certificate(cert.der())?;

    let signer = SigningKey::random(&mut rand::rngs::OsRng);
    let public_der = signer.verifying_key().to_public_key_der()?;
    let spki = x509_cert::spki::SubjectPublicKeyInfoOwned::from_der(public_der.as_bytes())?;
    let builder = CertificateBuilder::new(
        Profile::Leaf { issuer: Name::from_str("CN=Kenaz spike")?, enable_key_agreement: false, enable_key_encipherment: false },
        SerialNumber::from(1u32),
        Validity::from_now(Duration::from_secs(86400))?,
        Name::from_str("CN=Kenaz spike")?,
        spki,
        &signer,
    )?;
    let cert = builder.build::<p256::ecdsa::DerSignature>()?;
    verify_certificate(&cert.to_der()?)?;
    println!("rcgen and x509-cert P-256 self-signatures verified; SSH and hardware not exercised");
    Ok(())
}
