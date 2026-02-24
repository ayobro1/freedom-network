use rand::RngCore;
use sha3::{Digest, Sha3_256};
use hex::encode;

pub struct Identity {
    pub private_key: [u8;32],
    pub public_key: [u8;32],
    pub address: String,
}

pub fn generate_identity() -> Identity {
    let mut private_key = [0u8;32];
    rand::thread_rng().fill_bytes(&mut private_key);

    let public_key = Sha3_256::digest(&private_key);
    let mut hasher = Sha3_256::new();
    hasher.update(&public_key);
    let address = encode(hasher.finalize());

    Identity {
        private_key,
        public_key: public_key.into(),
        address,
    }
}