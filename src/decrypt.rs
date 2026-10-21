use ::base64::Engine;
use ::base64::engine::general_purpose::STANDARD;

use crate::encrypt::{ALGORITHMS, Algorithm, NONCE_LEN, SALT_LEN};

pub fn decrypt(args: &[String]) -> Result<String, String> {
    let usage = || format!("usage: it-tools decrypt <encrypted-text> <key> [{ALGORITHMS}]");
    let ([encrypted, key] | [encrypted, key, _]) = args else {
        return Err(usage());
    };
    let algorithm = Algorithm::parse(args.get(2)).ok_or_else(usage)?;

    let bytes = STANDARD.decode(encrypted).map_err(|err| format!("invalid base64: {err}"))?;
    if bytes.len() < SALT_LEN + NONCE_LEN {
        return Err("encrypted text is too short".into());
    }
    let (salt, rest) = bytes.split_at(SALT_LEN);
    let (nonce, ciphertext) = rest.split_at(NONCE_LEN);
    let plaintext = algorithm
        .apply(false, key, salt, nonce, ciphertext)
        .ok_or("decryption failed: wrong key, wrong algorithm or corrupted text")?;
    String::from_utf8(plaintext).map_err(|_| "decrypted data is not valid UTF-8 text".into())
}
