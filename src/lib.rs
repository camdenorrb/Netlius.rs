#![feature(unsafe_cell_get_mut)]
#![feature(async_closure)]

use async_std::sync::Arc;

use crate::net::client::Client;
use crate::net::server::Server;

pub mod net;
pub mod data;
pub mod async_utils;
pub mod extensions;

pub struct Netlius {}

impl Default for Netlius {
    fn default() -> Self {
        Netlius {}
    }
}

impl Netlius {

    // TODO: Store a threadpool in netlius and pass to Client and Server

    pub async fn client(&self, address: &str) -> Client {

        let mut client = Client::default();
        client.connect(address.parse().unwrap()).await.unwrap();

        client
    }

    pub async fn server(&self, address: &str) -> Server {

        let mut server = Server {
            address: address.to_string(),
            task: None,
            connect_listeners: Arc::new(Default::default()),
            disconnect_listeners: Arc::new(Default::default()),
            clients: Arc::new(Default::default())
        };

        server.start().await;

        server
    }

}
