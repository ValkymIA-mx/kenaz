use pgp::{
    composed::{DetachedSignature, EncryptionCaps, KeyType, SecretKeyParamsBuilder, SubkeyParamsBuilder},
    crypto::{ecc_curve::ECCCurve, hash::HashAlgorithm},
    types::{KeyVersion, Password},
};
use sequoia_openpgp::{cert::{CertBuilder, CipherSuite}, policy::StandardPolicy};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (cert, _) = CertBuilder::new()
        .set_cipher_suite(CipherSuite::Cv25519)
        .add_userid("Kenaz Spike <spike@example.invalid>")
        .add_signing_subkey()
        .add_transport_encryption_subkey()
        .add_authentication_subkey()
        .generate()?;
    assert!(cert.is_tsk());
    assert_eq!(cert.bad_signatures().count(), 0);
    let policy = StandardPolicy::new();
    let valid = cert.with_policy(&policy, None)?;
    assert_eq!(valid.userids().count(), 1);
    assert!(valid.keys().for_transport_encryption().count() >= 1);
    assert!(valid.keys().for_signing().count() >= 1);
    assert!(valid.keys().for_authentication().count() >= 1);

    let encryption = SubkeyParamsBuilder::default()
        .version(KeyVersion::V4)
        .key_type(KeyType::ECDH(ECCCurve::Curve25519Legacy))
        .can_encrypt(EncryptionCaps::All)
        .build()?;
    let signing = SubkeyParamsBuilder::default()
        .version(KeyVersion::V4)
        .key_type(KeyType::Ed25519Legacy)
        .can_sign(true)
        .build()?;
    let authentication = SubkeyParamsBuilder::default()
        .version(KeyVersion::V4)
        .key_type(KeyType::Ed25519Legacy)
        .can_authenticate(true)
        .build()?;
    let secret = SecretKeyParamsBuilder::default()
        .version(KeyVersion::V4)
        .key_type(KeyType::Ed25519Legacy)
        .can_certify(true)
        .can_sign(true)
        .primary_user_id("Kenaz Spike <spike@example.invalid>".to_owned())
        .subkey(encryption)
        .subkey(signing)
        .subkey(authentication)
        .build()?
        .generate(rand::thread_rng())?;
    secret.verify_bindings()?;
    let public = secret.to_public_key();
    let signature = DetachedSignature::sign_binary_data(
        rand::thread_rng(), &secret.primary_key, &Password::empty(),
        HashAlgorithm::Sha256, &b"kenaz spike"[..],
    )?;
    signature.verify(&public, b"kenaz spike")?;
    assert!(signature.verify(&public, b"changed").is_err());
    println!("Sequoia policy and rPGP bindings/signature checked in memory; card import and external acceptance not exercised");
    Ok(())
}
