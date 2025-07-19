extern crate console_error_panic_hook;
extern crate wasm_bindgen;

use rsa::{
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
    pkcs1v15::{Pkcs1v15Sign, SigningKey},
    pkcs8::{DecodePrivateKey, DecodePublicKey},
    sha2::{Digest, Sha256},
    signature::{RandomizedSigner, SignatureEncoding},
};
use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encrypt(data: &[u8], pub_key_pem: &str) -> Vec<u8> {
    init();

    let mut rng = rand::thread_rng();
    let pub_key =
        RsaPublicKey::from_public_key_pem(pub_key_pem).expect("failed to load public key");
    pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])
        .expect("failed to encrypt")
}

#[wasm_bindgen]
pub fn decrypt(data: &[u8], priv_key_pem: &str) -> Vec<u8> {
    init();

    let priv_key = RsaPrivateKey::from_pkcs8_pem(priv_key_pem).expect("failed to load private key");
    priv_key
        .decrypt(Pkcs1v15Encrypt, &data)
        .expect("failed to decrypt")
}

#[wasm_bindgen]
pub fn signature(data: &[u8], priv_key_pem: &str) -> Vec<u8> {
    init();

    let mut rng = rand::thread_rng();
    let priv_key = RsaPrivateKey::from_pkcs8_pem(priv_key_pem).expect("failed to load private key");
    let signing_key = SigningKey::<Sha256>::new(priv_key);
    let signature = signing_key.sign_with_rng(&mut rng, data);
    signature.to_bytes().to_vec()
}

#[wasm_bindgen]
pub fn verify(data: &[u8], signature: &[u8], pub_key_pem: &str) -> bool {
    init();

    let pub_key =
        RsaPublicKey::from_public_key_pem(pub_key_pem).expect("failed to load public key");
    let digest = Sha256::digest(data).to_vec();
    let result = pub_key.verify(Pkcs1v15Sign::new::<Sha256>(), &digest, signature);
    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}
