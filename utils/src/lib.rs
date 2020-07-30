use noiseexplorer_kk::types::{Keypair, PrivateKey, PublicKey};
use noiseexplorer_kk::consts::*;

use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io;

pub struct Array<T> {
    pub data: [T; 48]
}

impl<T: fmt::Debug> fmt::Debug for Array<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.data[..].fmt(formatter)
    }
}

pub fn load_static_keypair(entity: &str) -> Result<Keypair, io::Error> {
    let mut entity_rand_bytes = File::open(format!("../{}_random_bytes.txt", entity))?;
    let mut contents = vec![];
    entity_rand_bytes.read_to_end(&mut contents).unwrap();

    let mut rand_bytes = [0u8; DHLEN];
    rand_bytes.copy_from_slice(&contents[..DHLEN]);

    let static_privkey: PrivateKey = PrivateKey::from_bytes(rand_bytes);

    match Keypair::from_key(static_privkey) {
        Ok(keypair) => Ok(keypair),
        Err(e) => panic!("Error generating static key pair {:?}", e),
    }

}

pub fn load_remote_pubkey(entity: &str) -> Result<PublicKey, io::Error> {
    let mut entity_static_pubkey = File::open(format!("../{}_static_pubkey.txt", entity))?;
    let mut contents = vec![];
    entity_static_pubkey.read_to_end(&mut contents).unwrap();

    let mut pubkey_bytes = [0u8; DHLEN];
    pubkey_bytes.copy_from_slice(&contents[..DHLEN]);

    match PublicKey::from_bytes(pubkey_bytes) {
        Ok(public_key) => Ok(public_key),
        Err(e) => panic!("Error loading public key: {:?}", e),
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
