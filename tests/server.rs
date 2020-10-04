

#![feature(test)]
extern crate test;

use async_std::task::block_on;
use netlius::net::packet::Packet;
use test::Bencher;
use async_std::sync::Arc;


#[test]
fn basic_server_test() {
    block_on(async {

        let netlius = netlius::Netlius::default();

        let mut server = netlius.server("127.0.0.1:12345").await;
        let mut client = netlius.client("127.0.0.1:12345").await;

        // TODO: Future with parameter?
        server.on_connect(Arc::new(|client| {
            block_on(async move {
                client.write_and_flush_packet(Packet::default().utf8("Meow")).await;
            });
        })).await;

        client.write_and_flush_packet(Packet::default().utf8("Meow")).await;

        println!("{}", client.read_utf8().await.unwrap());
    });
}

// The overhead is mainly the kernel, I believe.
#[bench]
fn bench_server(bencher: &mut Bencher) {
    block_on(async {

        let netlius = netlius::Netlius::default();

        let mut server = netlius.server("127.0.0.1:12345").await;
        let mut client = netlius.client("127.0.0.1:12345").await;

        let packet_arc = Arc::new(Packet::default().u8(1));
        let packet_arc_clone = packet_arc.clone();

        server.on_connect(Arc::new(move |client| {

            let packet_arc = packet_arc_clone.clone();

            block_on(async move {
                loop {
                    assert_eq!(client.read_u8().await.unwrap(), 1);
                    client.write_and_flush_packet_arc(packet_arc.clone()).await;
                }
            });
        })).await;

        bencher.iter(|| {
            block_on(async {
                client.write_and_flush_packet_arc(packet_arc.clone()).await;
                assert_eq!(client.read_u8().await.unwrap(), 1);
            });
        });
    });
}
