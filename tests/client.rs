use netlius::Netlius;
use netlius::net::packet::Packet;

#[test]
fn send_basic_message() {

    let netlius = Netlius::default();

    let mut client  = async_std::task::block_on(netlius.client("127.0.0.1:12345"));
    async_std::task::block_on(client.write_and_flush(Packet::new().i32(1)));
}