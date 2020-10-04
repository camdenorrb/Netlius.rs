use crate::net::client::Client;

use async_std::net::{TcpListener, SocketAddr};
use async_std::task::{spawn, JoinHandle};

use async_std::sync::{Arc, Mutex};
use futures::{StreamExt};
use crate::async_utils::suspend::Suspend;
use crate::async_utils::holder::UnsafeHolder;
use futures::executor::block_on;
use std::ops::{Deref, DerefMut};
use std::borrow::BorrowMut;

pub struct Server {
    pub address: String,
    pub task: Option<JoinHandle<()>>,
    pub connect_listeners: Arc<Mutex<Vec<Arc<dyn Fn(&mut Client) + Send + Sync>>>>,
    pub disconnect_listeners: Arc<Mutex<Vec<Arc<dyn Fn(&Client) + Send + Sync>>>>,
    pub clients: Arc<Mutex<Vec<Arc<Mutex<Client>>>>>,
}

impl Server {

    // TODO: Use a threadpool for spawn
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
                    let mut client = client_arc.lock().await;

                    client.on_disconnect(Arc::new(move |client| {

                        let disconnect_listeners = disconnect_listeners.clone();

                        block_on(async {

                            disconnect_listeners.clone().lock().await.iter().for_each(|it| {
                                it(client)
                            });

                            clients.lock().await.deref_mut().retain(|it| {
                                block_on(async {
                                    it.lock().await.uuid != client.uuid
                                })
                            });
                        });

                    }));

                    // Connect listeners will handle the client
                    connect_listeners.lock().await.iter().for_each(|it| {
                        it.clone().deref()(&mut client)
                    });

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


    pub async fn on_connect(&mut self, listener: Arc<dyn Fn(&mut Client) + Send + Sync>) {
        self.connect_listeners.lock().await.push(listener);
    }

    pub async fn on_disconnect(&mut self, listener: Arc<dyn Fn(&Client) + Send + Sync>) {
        self.disconnect_listeners.lock().await.push(listener);
    }

}