use rand::rngs::OsRng;
use ed25519_dalek::{ Keypair, PublicKey, SecretKey };

fn get_key_pair() -> Keypair {
    
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);

    keypair

    /// Signature usage examples
    
    // Sign message
    // use ed25519_dalek::{Signature, Signer};
    // let message: &[u8] = b"This is a test of the tsunami alert system.";
    // let signature: Signature = keypair.sign(message);
    
    // Validate message's signature
    // use ed25519_dalek::Verifier;
    // assert!(keypair.verify(message, &signature).is_ok());

    // use ed25519_dalek::{PublicKey, Verifier};
    // let public_key: PublicKey = keypair.public;
    // assert!(public_key.verify(message, &signature).is_ok());

}

pub fn get_new_public_key() -> PublicKey {

    let key_pair: Keypair = get_key_pair();

    keypair.public
    
}

// Could be used we will need to save SecretKey
fn get_new_secret_key() -> SecretKey {

    let key_pair: Keypair = get_key_pair();

    keypair.secret
    
}