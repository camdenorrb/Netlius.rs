use crate::net::client::Client;

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
        client.connect(address.parse().unwrap()).await;

        client
    }

    pub async fn server(&self) {

    }

}
