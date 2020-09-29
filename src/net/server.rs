use crate::net::client::Client;
use async_std::io;
use async_std::net::{TcpListener, SocketAddr};
use async_std::prelude::*;
use async_std::task::spawn;
use std::borrow::{Borrow, BorrowMut};
use async_std::sync::{Arc, Weak};

pub struct Server {
    pub address: String,
    pub clients: Vec<Client>
}

impl Server {

    pub async fn start(&mut self) {

        let address = self.address.parse().unwrap();

        let listener = TcpListener::bind::<SocketAddr>(address).await.unwrap();
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            spawn(async move {

                let stream = stream.unwrap();
                let client = Client::new(stream);

                //weak.upgrade();//.unwrap().clients.push(client)
                self.clients.push(client);

                //let mut client = self.clients.last().unwrap();
                //let (reader, writer) = &mut (&client.tcp_stream.unwrap(), &client.tcp_stream.unwrap());

                //io::copy(reader, writer).await.unwrap();
            });
        }
    }

    pub fn stop(&self) {

    }


    pub fn on_connect(&self, listener: fn (&Client)) {

    }

    pub fn on_disconnect(&self, listener: fn (&Client)) {

    }

}