/// UNSUED contract

// use casper_types::PublicKey;

// pub fn get_new_public_key() -> PublicKey {

//     let new_public_key: PublicKey = PublicKey::generate_ed25519();

// }

// Options to create `PublicKey`

// pub fn from_bytes(bytes: &[u8]) -> Result<PublicKey, SignatureError>
// Construct a PublicKey from a slice of bytes.
// The caller is responsible for ensuring that the bytes passed into this method
// actually represent a curve25519_dalek::curve::CompressedEdwardsY
// and that said compressed point is actually a point on the curve.
// let bytes_curve25519: &[u8] = 
// let public_key_1 = from_bytes(bytes_curve25519);


// let hex_public_key: 
// let public_key = PublicKey::from_hex(&hex_public_key).map_err(|error| {
//     eprintln!("Can't parse {} as a public key: {}", hex_public_key, error);
//     Error::FailedToParseKey
// })?;

// ---

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

// ---

// use rand::rngs::OsRng;
// use ed25519_dalek::{ Keypair, PublicKey, SecretKey };

// let mut csprng = OsRng{};
// let keypair: Keypair = Keypair::generate(&mut csprng);

// fn get_key_pair() -> Keypair {
//     keypair
// }

// pub fn get_new_public_key() -> PublicKey {
//     let key_pair: Keypair = get_key_pair();
//     keypair.public
// }
// ---