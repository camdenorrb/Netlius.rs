use crate::net::client::Client;

use async_std::net::{TcpListener, SocketAddr};
use async_std::prelude::*;
use async_std::task::{spawn, JoinHandle};
use async_std::io;

pub struct Server {
    pub address: String,
    //pub clients: Arc<Mutex<Vec<Client>>>,
}

impl Server {

    // TODO: Maybe move self
    pub async fn start(&mut self) -> JoinHandle<()> {

        let address = self.address.parse().unwrap();
        //let clients = self.clients.clone();

        let job = spawn(async move {

            let listener = TcpListener::bind::<SocketAddr>(address).await.unwrap();
            let mut incoming = listener.incoming();

            while let Some(stream) = incoming.next().await {

                spawn(async move {

                    let stream = stream.unwrap();
                    let (reader, writer) = &mut (&stream, &stream);

                    loop {
                        io::copy(reader, writer).await.unwrap();
                    }
                });

                //let client = Client::new(stream);
                //clients.lock().await.push(client);
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