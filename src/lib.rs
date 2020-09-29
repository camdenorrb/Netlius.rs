//#![feature(async_closure)]

use crate::net::client::Client;
use crate::net::server::Server;
use async_std::net::TcpListener;
use async_std::net::SocketAddr;

pub mod net;
pub mod data;
pub mod extensions;

pub struct Netlius {}

impl Default for Netlius {
    fn default() -> Self {
        Netlius {}
    }
}

impl Netlius {

    pub async fn client(&self, address: &str) -> Client {

        let mut client = Client::default();
        client.connect(address.parse().unwrap()).await.unwrap();

        client
    }

    pub async fn server(&self, address: &str) -> Server {

        let mut server = Server {
            address: address.to_string(),
            clients: vec![]
        };

        server.start().await;

        server
    }

}
