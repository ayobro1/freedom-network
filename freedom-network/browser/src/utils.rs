use chacha20poly1305::{aead::{Aead, KeyInit}, ChaCha20Poly1305, Key, Nonce};
use sha3::{Digest, Sha3_256};
use hex;

/// Decrypts a `.fdom` file in memory
/// `ciphertext` includes 12-byte nonce + encrypted data
pub fn decrypt_fdom(ciphertext: &[u8], key_bytes: &[u8;32]) -> Vec<u8> {
    let (nonce_bytes, ct) = ciphertext.split_at(12);
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key_bytes));
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher.decrypt(nonce, ct).expect("Decryption failed")
}

/// Compute SHA3-256 hash of content (used for address verification)
pub fn hash_content(content: &[u8]) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(content);
    hex::encode(hasher.finalize())
}

/// Verify a site signature (placeholder)
/// In practice, you would use public key signature verification
pub fn verify_signature(_content: &[u8], _signature: &[u8], _public_key: &[u8]) -> bool {
    // TODO: implement post-quantum signature verification
    true
}

/// Convert a `.fdom` address (hex) to readable string
pub fn format_address(hex_addr: &str) -> String {
    format!("freedom://{}", hex_addr)
}
``