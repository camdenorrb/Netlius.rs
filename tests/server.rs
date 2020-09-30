use async_std::task::block_on;
use Netlius::net::packet::Packet;

#[test]
fn basic_server_test() {

    block_on(async {

        let netlius = Netlius::Netlius {};
        let server  = netlius.server("127.0.0.1:12345").await;

        for x in 0..1_000 {
            let mut client = netlius.client("127.0.0.1:12345").await;
            client.write_and_flush(Packet::new().u8(1)).await;
            client.read_u8().await.unwrap();
        }
    });

}