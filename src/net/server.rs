use crate::net::client::Client;

use async_std::net::{TcpListener, SocketAddr};
use async_std::task::{spawn};

use async_std::sync::{Arc, Mutex};
use futures::{StreamExt, AsyncReadExt};
use async_std::io;
use crate::async_utils::suspend::Suspend;
use crate::async_utils::holder::Holder;


pub struct Server {
    pub address: String,
    pub clients: Arc<Mutex<Vec<Arc<Mutex<Client>>>>>,
}

impl Server {

    // TODO: Use a threadpool for spawn
    pub async fn start(&mut self) {

        let address = self.address.parse().unwrap();
        let clients = self.clients.clone();

        let server_start_suspend = Arc::new(Holder::new(Suspend::new(true)));
        let server_start_suspend_clone = server_start_suspend.clone();

        spawn(async move {

            let clients = clients.clone();
            let listener = TcpListener::bind::<SocketAddr>(address).await.unwrap();
            let incoming = listener.incoming();

            unsafe {
                server_start_suspend_clone.get_mut().unsuspend().await;
            }

            incoming.for_each_concurrent(None, |stream| async {
                let clients = clients.clone();
                let client_arc = Arc::new(Mutex::new(Client::new(stream.unwrap())));

                clients.lock().await.push(client_arc.clone());

                spawn(async move {

                    // TODO: Call connect listeners here and remove rest

                    let client_arc = client_arc.clone();
                    let client = client_arc.lock().await;
                    let (mut reader, mut writer) = client.tcp_stream.as_ref().unwrap().split();

                    loop {
                        io::copy(&mut reader, &mut writer).await.unwrap();
                    }
                });
            }).await;
        });

        unsafe {
            server_start_suspend.get_mut().await;
        }
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