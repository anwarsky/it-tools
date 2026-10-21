use ::base64::Engine;
use ::base64::engine::general_purpose::STANDARD;
use aes_gcm::Aes256Gcm;
use aes_gcm::aead::{Aead, KeyInit, Nonce};
use chacha20poly1305::ChaCha20Poly1305;

pub const ALGORITHMS: &str = "AES|CHACHA20";
pub const SALT_LEN: usize = 16;
pub const NONCE_LEN: usize = 12;
const KDF_ROUNDS: u32 = 600_000;
const USAGE: &str = "usage: it-tools encrypt <text> <key> [AES|CHACHA20]
  AES       AES-256-GCM (default)
  CHACHA20  ChaCha20-Poly1305
  the key is a passphrase; output is base64(salt | nonce | ciphertext)";

pub enum Algorithm {
    Aes,
    ChaCha20,
}

impl Algorithm {
    pub fn parse(name: Option<&String>) -> Option<Self> {
        match name.map(|n| n.to_uppercase()).as_deref() {
            None | Some("AES") => Some(Self::Aes),
            Some("CHACHA20") => Some(Self::ChaCha20),
            _ => None,
        }
    }

    /// Encrypts (`seal`) or decrypts (`!seal`) `data` with a key derived from `passphrase` and `salt`.
    pub fn apply(&self, seal: bool, passphrase: &str, salt: &[u8], nonce: &[u8], data: &[u8]) -> Option<Vec<u8>> {
        let mut key = [0u8; 32];
        pbkdf2::pbkdf2_hmac::<sha2::Sha256>(passphrase.as_bytes(), salt, KDF_ROUNDS, &mut key);
        match self {
            Self::Aes => run::<Aes256Gcm>(seal, &key, nonce, data),
            Self::ChaCha20 => run::<ChaCha20Poly1305>(seal, &key, nonce, data),
        }
    }
}

fn run<A: Aead + KeyInit>(seal: bool, key: &[u8], nonce: &[u8], data: &[u8]) -> Option<Vec<u8>> {
    let cipher = A::new_from_slice(key).ok()?;
    let nonce = Nonce::<A>::try_from(nonce).ok()?;
    if seal { cipher.encrypt(&nonce, data) } else { cipher.decrypt(&nonce, data) }.ok()
}

pub fn encrypt(args: &[String]) -> Result<String, String> {
    let ([text, key] | [text, key, _]) = args else {
        return Err(USAGE.into());
    };
    let algorithm = Algorithm::parse(args.get(2)).ok_or_else(|| USAGE.to_string())?;

    let salt: [u8; SALT_LEN] = rand::random();
    let nonce: [u8; NONCE_LEN] = rand::random();
    let ciphertext = algorithm
        .apply(true, key, &salt, &nonce, text.as_bytes())
        .ok_or("encryption failed")?;
    Ok(STANDARD.encode([&salt[..], &nonce, &ciphertext].concat()))
}
