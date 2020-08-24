use noiseexplorer_kk::noisesession::{NoiseSession};
use noiseexplorer_kk::types::{Keypair};
use noiseexplorer_kk::consts::*;

use tokio::net::{TcpListener};
use tokio::prelude::*;

use utils::{print_msg, load_static_keypair, load_remote_pubkey, handle_hs_read, HS_MSG_LEN};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let s =  load_static_keypair("server").unwrap();
    let rs = load_remote_pubkey("client").unwrap();
    let mut session = NoiseSession::init_session(false, &[0u8], s, Some(rs));

    let mut listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("Server running on localhost:8000");

    match listener.accept().await {
        Ok((mut socket, addr)) => {
            println!("New client: {:?}", addr);
            let (reader, mut writer) = socket.split();

            // Handle Read
            let mut msg_a = handle_hs_read(reader).await;
            print_msg(&msg_a, false);
            session.recv_message(&mut msg_a).unwrap();

            // Handle Write
            let e_keypair = Keypair::default();

            let e = e_keypair.get_public_key().as_bytes();
            let mut in_out = [0u8; HS_MSG_LEN];
            in_out[..DHLEN].copy_from_slice(&e);

            session.set_ephemeral_keypair(e_keypair);
            session.send_message(&mut in_out).unwrap(); 

            let result = writer.write(&in_out).await;
            print_msg(&in_out, true);
            println!("wrote to stream; success={:?}", result.is_ok());
        }

        Err(e) => panic!("couldn't get client: {:?}", e),
    };

    println!("message count = {:?}", session.get_message_count());
    println!("handshake hash = {:?}", session.get_handshake_hash());
    println!("is transport? {:?}", session.is_transport());

    Ok(())

    // Following this, set up a loop where the server awaits for messages from the client and responds accordingly.
}

