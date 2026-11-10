use std::collections::HashSet;

use age::{DecryptError, EncryptError, Identity, Recipient};
use age_core::format::{FileKey, Stanza};

// These wrappers exercise age's public extension points, not a hardware protocol.
struct WrappedRecipient(age::x25519::Recipient);
struct WrappedIdentity(age::x25519::Identity);

impl Recipient for WrappedRecipient {
    fn wrap_file_key(&self, key: &FileKey) -> Result<(Vec<Stanza>, HashSet<String>), EncryptError> {
        let (mut stanzas, labels) = self.0.wrap_file_key(key)?;
        for stanza in &mut stanzas {
            assert_eq!(stanza.tag, "X25519");
            stanza.tag = "kenaz-spike-X25519".to_owned();
        }
        Ok((stanzas, labels))
    }
}

impl Identity for WrappedIdentity {
    fn unwrap_stanza(&self, stanza: &Stanza) -> Option<Result<FileKey, DecryptError>> {
        if stanza.tag != "kenaz-spike-X25519" {
            return None;
        }
        let native = Stanza {
            tag: "X25519".to_owned(),
            args: stanza.args.clone(),
            body: stanza.body.clone(),
        };
        self.0.unwrap_stanza(&native)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = age::x25519::Identity::generate();
    let recipient = WrappedRecipient(key.to_public());
    let identity = WrappedIdentity(key);
    let other = WrappedIdentity(age::x25519::Identity::generate());
    let plaintext = b"in-memory age boundary experiment";
    let mut encrypted = age::encrypt(&recipient, plaintext)?;
    assert_eq!(age::decrypt(&identity, &encrypted)?, plaintext);
    assert!(age::decrypt(&other, &encrypted).is_err());
    let end = encrypted.len() - 1;
    encrypted[end] ^= 1;
    assert!(age::decrypt(&identity, &encrypted).is_err());
    println!("custom age stanza roundtrip, wrong identity and tamper rejection passed; no hardware or plugin exercised");
    Ok(())
}
