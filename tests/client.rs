use async_std::task::block_on;

use netlius::net::client::Client;
use netlius::net::packet::Packet;
use netlius::Netlius;

#[test]
fn send_basic_message() {
    block_on(async {

        let netlius = Netlius::default();
        let mut client = netlius.client("127.0.0.1:12345").await;

        client.write_and_flush_packet(Packet::default().utf8("Meow")).await;

        println!("{}", client.read_utf8().await.unwrap());
    });
}