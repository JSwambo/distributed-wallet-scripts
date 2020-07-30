use noiseexplorer_kk::noisesession::{NoiseSession};
use noiseexplorer_kk::types::{Keypair};
use noiseexplorer_kk::consts::*;

use tokio::net::TcpStream;
use tokio::prelude::*;

use utils::{Array, load_static_keypair, load_remote_pubkey};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    println!("created stream");

    let s = load_static_keypair("client").unwrap();

    let rs = load_remote_pubkey("server").unwrap();

    let mut session = NoiseSession::init_session(true, &[0u8], s, Some(rs));

    let e_keypair = Keypair::default();

    let e = e_keypair.get_public_key().as_bytes();
    let mut in_out = [0u8; 48];
    in_out[..DHLEN].copy_from_slice(&e);
    println!("(before processing) in_out = {:?}", Array{ data: in_out });

    session.send_message(&mut in_out); 

    println!("(after processing) in_out = {:?}", Array{ data: in_out });

    let result = stream.write(&in_out).await;
    println!("wrote to stream; success={:?}", result.is_ok());

    println!("message count = {:?}", session.get_message_count());
    println!("handshake hash = {:?}", session.get_handshake_hash());
}