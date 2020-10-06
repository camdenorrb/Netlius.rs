#![feature(async_closure)]

#![feature(test)]
extern crate test;

use test::Bencher;

use async_std::pin::Pin;
use async_std::sync::Arc;
use async_std::task::block_on;

use netlius::net::packet::Packet;

#[test]
fn basic_server_test() {
    block_on(async {

        let netlius = netlius::Netlius::default();

        let mut server = netlius.server("127.0.0.1:12345").await;
        let mut client = netlius.client("127.0.0.1:12345").await;

        // TODO: Future with parameter?
        server.on_connect(Box::new(|client| Box::pin(async move {
            block_on(async move {
                client.write_and_flush_packet(Packet::default().utf8("Meow")).await;
            });
        }))).await;

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

        server.on_connect(Box::new(move |client| {

            let packet_arc_clone = packet_arc_clone.clone();

            Box::pin(async move {
                println!("Here");
                loop {
                    assert_eq!(client.read_u8().await.unwrap(), 1);
                    client.write_and_flush_packet_arc(packet_arc_clone.clone()).await;
                };
            })
        })).await;

        bencher.iter(|| {
            block_on(async {
                client.write_and_flush_packet_arc(packet_arc.clone()).await;
                assert_eq!(client.read_u8().await.unwrap(), 1);
            });
        });
    });
}
