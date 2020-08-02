use noiseexplorer_kk::noisesession::{NoiseSession};
use noiseexplorer_kk::types::{Keypair};
use noiseexplorer_kk::consts::*;

use tokio::net::TcpStream;
use tokio::prelude::*;

use utils::{print_msg, load_static_keypair, load_remote_pubkey, handle_hs_read, HS_MSG_LEN};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    println!("created stream");

    let s = load_static_keypair("client").unwrap();
    let rs = load_remote_pubkey("server").unwrap();
    let mut session = NoiseSession::init_session(true, &[0u8], s, Some(rs));

    // Handle Write
    let e_keypair = Keypair::default();

    let e = e_keypair.get_public_key().as_bytes();
    let mut in_out = [0u8; HS_MSG_LEN];
    in_out[..DHLEN].copy_from_slice(&e);

    session.set_ephemeral_keypair(e_keypair);
    session.send_message(&mut in_out).unwrap(); //processes in_out

    stream.write(&in_out).await.unwrap();
    print_msg(&in_out, true);

    // Handle Read
    let (reader, mut _writer) = stream.split();

    let mut msg_b = handle_hs_read(reader).await;
    print_msg(&msg_b, false);
    session.recv_message(&mut msg_b).unwrap();

    println!("message count = {:?}", session.get_message_count());
    println!("handshake hash = {:?}", session.get_handshake_hash());
    println!("is transport? {:?}", session.is_transport());
}