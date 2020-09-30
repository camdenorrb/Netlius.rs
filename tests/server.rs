#![feature(test)]

extern crate test;

use async_std::task::block_on;
use Netlius::net::packet::Packet;
use test::Bencher;
use std::hint::black_box;

#[test]
fn basic_server_test() {
    block_on(async {

        let netlius = Netlius::Netlius {};

        let server     = netlius.server("127.0.0.1:12345").await;
        let mut client = netlius.client("127.0.0.1:12345").await;

        client.write_and_flush(Packet::new().utf8("Meow")).await;

        println!("{}", client.read_utf8().await.unwrap());
    });
}

#[bench]
fn bench_server(bencher: &mut Bencher) {

    let netlius = Netlius::Netlius {};

    let server = block_on(netlius.server("127.0.0.1:12345"));
    let mut client = block_on(netlius.client("127.0.0.1:12345"));

    bencher.iter(|| {
        block_on(async {

            let n: i32 = black_box(100_000);

            for i in 0..n {
                client.write_and_flush(Packet::new().utf8("Meow")).await;
                client.read_utf8().await.unwrap();
            }
        });
    });
}
