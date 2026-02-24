use chacha20poly1305::{aead::{Aead, KeyInit}, ChaCha20Poly1305, Key, Nonce};
use rand::RngCore;

pub fn encrypt(data: &[u8], key_bytes: &[u8;32]) -> Vec<u8> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key_bytes));
    let mut nonce_bytes = [0u8;12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let mut ciphertext = cipher.encrypt(nonce, data).unwrap().to_vec();
    let mut result = nonce_bytes.to_vec();
    result.append(&mut ciphertext);
    result
}

pub fn decrypt(ciphertext: &[u8], key_bytes: &[u8;32]) -> Vec<u8> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key_bytes));
    let (nonce_bytes, ct) = ciphertext.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher.decrypt(nonce, ct).unwrap()
}