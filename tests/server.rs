#![feature(test)]
extern crate test;

use async_std::task::block_on;
use netlius::net::packet::Packet;
use test::Bencher;
use std::hint::black_box;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn basic_server_test() {
    block_on(async {

        let netlius = netlius::Netlius {};

        let server     = netlius.server("127.0.0.1:12345").await;
        let mut client = netlius.client("127.0.0.1:12345").await;

        client.write_and_flush(Packet::default().utf8("Meow")).await;

        println!("{}", client.read_utf8().await.unwrap());
    });
}

#[bench]
fn bench_server(bencher: &mut Bencher) {

    let netlius = netlius::Netlius {};

    let server = block_on(netlius.server("127.0.0.1:12345"));
    let mut client = block_on(netlius.client("127.0.0.1:12345"));

    bencher.iter(|| {
        block_on(async {
            client.write_and_flush(Packet::default().utf8("Meow")).await;
            client.read_utf8().await.unwrap();
        });
    });
}
