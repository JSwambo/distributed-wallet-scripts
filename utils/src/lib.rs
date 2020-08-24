use std::net::SocketAddr;
use anyhow::Result;
use tokio::net::{TcpStream};
use tokio_socks::tcp::Socks5Stream;
use tokio_socks::IntoTargetAddr;

use tokio::prelude::*;
use tokio::net::{tcp::ReadHalf};

use noiseexplorer_kk::types::{Keypair, PrivateKey, PublicKey};
use noiseexplorer_kk::consts::*;

use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io;

pub const HS_MSG_LEN: usize = 128;


pub async fn connect_tor_socks_proxy<'a>(proxy: SocketAddr, dest: impl IntoTargetAddr<'a>) -> Result<TcpStream> {
    let sock = Socks5Stream::connect(proxy, dest).await?;
    Ok(sock.into_inner())
}

pub struct Array<T> {
    pub data: [T; HS_MSG_LEN]
}

impl<T: fmt::Debug> fmt::Debug for Array<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.data[..].fmt(formatter)
    }
}

pub fn print_msg(msg: &[u8; HS_MSG_LEN], sent: bool) {
    let mut printable_msg = [0u8; HS_MSG_LEN];
    printable_msg.copy_from_slice(&msg[..]);
    if let true = sent {
        println!("Sent message: {:?}", Array{ data: printable_msg });        
    } else {
        println!("Received message: {:?}", Array{ data: printable_msg });     
    };
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


pub async fn handle_hs_read(mut reader: ReadHalf<'_>) -> [u8; HS_MSG_LEN] {
    let mut msg_buf = [0u8; HS_MSG_LEN];
    reader.read(&mut msg_buf).await.unwrap();
    msg_buf
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

