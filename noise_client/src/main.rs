use noiseexplorer_kk::noisesession::{NoiseSession};
use noiseexplorer_kk::types::{Keypair, PrivateKey, PublicKey};
use noiseexplorer_kk::consts::*;

use tokio::net::TcpStream;
use tokio::prelude::*;

use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() {
    // let mut stream = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    // println!("created stream");

    let s = load_static_keypair().unwrap();

    let rs = load_remote_pubkey().unwrap();

    let mut session = NoiseSession::init_session(true, &[0u8], s, Some(rs));

    let e_keypair = Keypair::default();

    let mut in_out: [u8; DHLEN] = e_keypair.get_public_key().as_bytes();
    println!("in_out = {:?}", in_out);

    // TODO: Panics here, but why?
    // session.send_message(&mut in_out); 

    // let result = stream.write(&in_out).await;
    // println!("wrote to stream; success={:?}", result.is_ok());

    // println!("message count = {:?}", session.get_message_count());
    // println!("handshake hash = {:?}", session.get_handshake_hash());
}


fn load_static_keypair() -> Result<Keypair, io::Error> {
    let mut server_rand_bytes = File::open("../client_random_bytes.txt")?;
    let mut contents = vec![];
    server_rand_bytes.read_to_end(&mut contents).unwrap();

    let mut rand_bytes = [0u8; DHLEN];
    rand_bytes.copy_from_slice(&contents[..DHLEN]);

    let static_privkey: PrivateKey = PrivateKey::from_bytes(rand_bytes);

    match Keypair::from_key(static_privkey) {
        Ok(keypair) => Ok(keypair),
        Err(e) => panic!("Error generating static key pair {:?}", e),
    }

}


fn load_remote_pubkey() -> Result<PublicKey, io::Error> {
    let mut client_static_pubkey = File::open("../server_static_pubkey.txt")?;
    let mut contents = vec![];
    client_static_pubkey.read_to_end(&mut contents).unwrap();

    let mut pubkey_bytes = [0u8; DHLEN];
    pubkey_bytes.copy_from_slice(&contents[..DHLEN]);

    match PublicKey::from_bytes(pubkey_bytes) {
        Ok(public_key) => Ok(public_key),
        Err(e) => panic!("Error loading public key: {:?}", e),
    }
}