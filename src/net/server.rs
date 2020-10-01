use crate::net::client::Client;

use async_std::net::{TcpListener, SocketAddr};
use async_std::prelude::*;
use async_std::task::{spawn, JoinHandle};
use futures::StreamExt;


pub struct Server {
    pub address: String,
    pub clients: Vec<Client>,
}

impl Server {

    // TODO: Maybe move self
    pub async fn start(&mut self) -> JoinHandle<()> {

        let thing = Arc::RefCell::new(self);
        "".to_owned();


        let address = self.address.parse().unwrap();
        //let clients = self.clients.clone();

        let job = async_std::task::spawn(async move {

            let listener = TcpListener::bind::<SocketAddr>(address).await.unwrap();
            let mut incoming = listener.incoming();
            incoming.for_each_concurrent()

            let mut clients = self.clients;

            while let Some(stream) = incoming.next().await {

                let client = Client::new(stream.unwrap());

                clients.push(client);

                let client = clients.last_mut().unwrap();

                spawn(async move {

                    // TODO: Call connect listeners here and remove rest

                    let stream = client.tcp_stream.as_ref().unwrap();
                    let (reader, writer) = &mut (stream, stream);

                    loop {
                        io::copy(reader, writer).await.unwrap();
                    }
                });


            }
        });

        return job;
    }

    /*
    pub fn stop(&self) {

    }
    */


    pub fn on_connect(&self, listener: fn (&Client)) {

    }

    pub fn on_disconnect(&self, listener: fn (&Client)) {

    }

}