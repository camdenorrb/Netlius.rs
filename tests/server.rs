use netlius::Netlius;
use netlius::net::packet::Packet;

#[test]
fn thing() {

    let netlius = Netlius::default();

    loop {
        let mut client  = async_std::task::block_on(netlius.client("127.0.0.1:12345"));
        async_std::task::block_on(client.write_and_flush(Packet::new().i32(1)));
    }
}