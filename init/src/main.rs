// init is used to simulate the out-of-band ceremony between participants where their static public keys are shared. 

use noiseexplorer_kk::types::{Keypair, PrivateKey};
use noiseexplorer_kk::consts::*;

use std::fs::File;
use std::io::prelude::*;
use std::io;

use rand::RngCore;


fn main() {
    let entities = ["server","client"];

    for entity in entities.iter() {
        match static_keypair_existence_check(entity) {
             Ok(_) => {
                println!("Found existing static keypair for {}", entity);
                Keypair::new_empty()
            }
             Err(_) => {
                println!("Generated static keypair for {}", entity);
                generate_static_keypair(entity).unwrap()
            }
        };
    }
}

fn generate_static_keypair(entity: &str) -> Result<Keypair, io::Error> {
    //create an array of random bytes and generate static private and public key pairs.
    let mut rand_bytes = [0u8; DHLEN];
    rand::thread_rng().fill_bytes(&mut rand_bytes);

    //_WARNING: ONLY USED FOR TESTING PURPOSES_
    // save rand_bytes in a file 
    let mut file = File::create(format!("../{}_random_bytes.txt", entity))?;

    file.write_all(&rand_bytes)?;

    let static_privkey: PrivateKey = PrivateKey::from_bytes(rand_bytes);

    let static_pubkey = match static_privkey.generate_public_key() {
        Ok(public_key) => public_key,
        Err(e) => panic!("Error generating public key: {:?}", e),
    };

    // write the static pubkey to a file so it can be shared out-of-band. 
    let mut file = File::create(format!("../{}_static_pubkey.txt", entity))?;

    file.write_all(&static_pubkey.as_bytes())?;

    match Keypair::from_key(static_privkey) {
        Ok(keypair) => Ok(keypair),
        Err(e) => panic!("Error generating static key pair {:?}", e),
    }
}

fn static_keypair_existence_check(entity: &str) -> Result<bool, io::Error> {
    File::open(format!("../{}_random_bytes.txt", entity))?; 
    File::open(format!("../{}_static_pubkey.txt", entity))?;
    Ok(true)
}
