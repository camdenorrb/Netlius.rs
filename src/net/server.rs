use std::ops::{DerefMut, Deref};

use async_std::net::{SocketAddr, TcpListener};
use async_std::sync::{Arc, Mutex};
use async_std::task::{JoinHandle, spawn};
use futures::executor::block_on;
use futures::future::BoxFuture;
use futures::StreamExt;

use crate::async_utils::holder::UnsafeHolder;
use crate::async_utils::suspend::Suspend;
use crate::net::client::Client;

pub struct Server {
    pub address: String,
    pub task: Option<JoinHandle<()>>,
    pub connect_listeners: Arc<Mutex<Vec<Box<dyn Fn(&mut Client) -> BoxFuture<()> + Send + Sync>>>>,
    pub disconnect_listeners: Arc<Mutex<Vec<Box<dyn Fn(&Client) -> BoxFuture<()> + Send + Sync>>>>,
    pub clients: Arc<Mutex<Vec<Arc<Mutex<Client>>>>>,
}

impl Server {

    // TODO: Use a threadpool for spawn
    // TODO: Use Result
    // TODO: Make Address more flexible like the exact argument that TcpStream#connect takes
    pub async fn start(&mut self) {

        let address = self.address.parse().unwrap();
        let clients = self.clients.clone();

        let connect_listeners = self.connect_listeners.clone();
        let disconnect_listeners = self.disconnect_listeners.clone();

        let server_start_suspend = Arc::new(UnsafeHolder::new(Suspend::new(true)));
        let server_start_suspend_clone = server_start_suspend.clone();

        let task = spawn(async move {

            let clients = clients.clone();

            let connect_listeners = connect_listeners.clone();
            //let disconnect_listeners = disconnect_listeners.clone();

            let listener = TcpListener::bind::<SocketAddr>(address).await.unwrap();
            let incoming = listener.incoming();

            unsafe {
                server_start_suspend_clone.get_mut().unsuspend().await;
            }

            incoming.for_each_concurrent(None, |stream| async {

                let clients = clients.clone();
                let client_arc = Arc::new(Mutex::new(Client::new(stream.unwrap())));

                let connect_listeners = connect_listeners.clone();
                let disconnect_listeners = disconnect_listeners.clone();

                clients.lock().await.push(client_arc.clone());

                spawn(async move {

                    let clients = clients.clone();
                    let connect_listeners = connect_listeners.clone();
                    let disconnect_listeners = disconnect_listeners.clone();

                    let client_arc = client_arc.clone();
                    let mut client = client_arc.clone().lock().await;

                    client.on_disconnect(Arc::new(|client| {

                        let disconnect_listeners = disconnect_listeners.clone();

                        let client_arc_clone = client_arc.clone();

                        spawn(async move {

                            disconnect_listeners.clone().lock().await.iter().for_each(|it| {
                                block_on(async {
                                    it(client_arc_clone.lock().await.deref()).await;
                                });
                            });

                            clients.lock().await.deref_mut().retain(move |it| {

                                let client_arc_clone = client_arc_clone.clone();

                                block_on(async move {
                                    true
                                    //it.lock().await.uuid != true//client.uuid
                                })
                            });

                        });

                    }));

                    // Connect listeners will handle the client
                    for connect_listener in connect_listeners.lock().await.deref_mut() {
                        connect_listener(&mut client).await;
                    }
                });
            }).await;
        });

        unsafe {
            server_start_suspend.get_mut().await;
        }

        self.task = Some(task);
    }

    pub async fn stop(self) {

        self.clients.lock().await.iter().for_each(|it| {
            block_on(async {
                it.lock().await.disconnect().await;
            });
        });

        self.task.expect("Couldn't retrieve server task").cancel().await;
    }


    pub async fn on_connect(&mut self, listener: Box<dyn Fn(&mut Client) -> BoxFuture<()> + Send + Sync>) {
        self.connect_listeners.lock().await.push(listener);
    }

    pub async fn on_disconnect(&mut self, listener: Box<dyn Fn(&Client) -> BoxFuture<()> + Send + Sync>) {
        self.disconnect_listeners.lock().await.push(listener);
    }

}