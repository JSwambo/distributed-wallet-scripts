use noiseexplorer_kk::noisesession::{NoiseSession};

use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use utils::{Array, load_static_keypair, load_remote_pubkey};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let s =  load_static_keypair("server").unwrap();

    let rs = load_remote_pubkey("client").unwrap();

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

    let mut printable_msg = [0u8; 48];
    printable_msg.copy_from_slice(&msg[..]);
    println!("revieved message: {:?}", Array{ data: printable_msg });

    //TODO: Decryption error with received message?
    match session.recv_message(&mut msg) {
         Ok(_) => println!("Message read"),
         Err(e) => panic!("Noise Error: {:?}", e),
     };
    
    println!("message count = {:?}", session.get_message_count());
    println!("handshake hash = {:?}", session.get_handshake_hash());

    Ok(())
}

async fn handle_client(mut socket: TcpStream) -> [u8;48] {
    let (mut reader, mut _writer) = socket.split();
    println!("Handling Client!");
    //TODO: this handler is specific to handshake messages and is not suitable for transport messages (with variable length)
    let mut msg_buf = [0u8;48];
    reader.read(&mut msg_buf).await;
    msg_buf
}