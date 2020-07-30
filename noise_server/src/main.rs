use noiseexplorer_kk::noisesession::{NoiseSession};
use noiseexplorer_kk::types::{Keypair, PrivateKey, PublicKey};
use noiseexplorer_kk::consts::*;

use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let s =  load_static_keypair().unwrap();

    let rs = load_remote_pubkey().unwrap();

    let mut session = NoiseSession::init_session(true, &[0u8], s, Some(rs));

    let mut listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();
    println!("Server running on localhost:6142");

    let mut msg = match listener.accept().await {
        Ok((_socket, addr)) => {
            println!("new client: {:?}", addr);
            handle_client(_socket).await
        }
        Err(e) => panic!("couldn't get client: {:?}", e),
    };

    println!("revieved message: {:?}", msg);
    session.recv_message(&mut msg);

    println!("message count = {:?}", session.get_message_count());
    println!("handshake hash = {:?}", session.get_handshake_hash());

    Ok(())
}

async fn handle_client(mut socket: TcpStream) -> [u8;DHLEN] {
    let (mut reader, mut _writer) = socket.split();
    println!("Handling Client!");
    //TODO: this handler is specific to handshake messages and is not suitable for transport messages (with variable length)
    let mut msg_buf = [0u8;DHLEN];
    reader.read(&mut msg_buf).await;
    msg_buf
}

fn load_static_keypair() -> Result<Keypair, io::Error> {
    let mut server_rand_bytes = File::open("../server_random_bytes.txt")?; //if this fails, return the error to the calling function
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
    let mut client_static_pubkey = File::open("../client_static_pubkey.txt")?;
    let mut contents = vec![];
    client_static_pubkey.read_to_end(&mut contents).unwrap();

    let mut pubkey_bytes = [0u8; DHLEN];
    pubkey_bytes.copy_from_slice(&contents[..DHLEN]);

    match PublicKey::from_bytes(pubkey_bytes) {
        Ok(public_key) => Ok(public_key),
        Err(e) => panic!("Error loading public key: {:?}", e),
    }
}