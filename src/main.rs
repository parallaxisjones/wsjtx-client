//! An UDP echo server that just sends back everything that it receives.
//!
//! If you're on Unix you can test this out by in one terminal executing:
//!
//!     cargo run --example echo-udp
//!
//! and in another terminal you can run:
//!
//!     cargo run --example connect -- --udp 127.0.0.1:8080
//!
//! Each line you type in to the `nc` terminal should be echo'd back to you!

#![warn(rust_2018_idioms)]

use std::net::Ipv4Addr;
use std::net::IpAddr;
use std::error::Error;
use std::net::SocketAddr;
use std::{env, io};
use tokio::net::UdpSocket;

struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
}

impl Server {
    pub async fn run(self) -> Result<(), io::Error> {
        let Server {
            socket,
            mut buf,
        } = self;
        socket.connect("172.18.144.1:2237".to_string()).await?;
        match socket.set_broadcast(true) {
            _ => println!("Set Broadcast"),
        };
        let us = socket.recv(&mut buf).await?;
        println!("recv {}", us);

        Ok(())
    }

    async fn new(addr: String) -> Result<Server, Box<dyn Error>> {
        let socket = UdpSocket::bind(&addr).await?;
        // let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 2237);
        Ok(Server {
            socket,
            buf: vec![0u8; 1024],
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let server = Server::new(addr).await?;    // This starts the server task.
    server.run().await?;

    Ok(())
}