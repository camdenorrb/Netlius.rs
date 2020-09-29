use enum_map::{enum_map, Enum, EnumMap};
use async_std::net::{TcpStream, Shutdown};
use std::borrow::BorrowMut;
use crate::net::packet::Packet;
use crate::net::buffer::ByteBuffer;
use std::mem;
use crate::extensions::bytes::BytesExt;
use async_std::net::{SocketAddr};
use async_std::prelude::*;
use async_std::io::Result;

type ClientListener = fn(client: &Client);

#[derive(Debug, Enum)]
enum ClientEvent {
    Connect,
    Disconnect
}

// https://docs.rs/async-std/0.99.4/async_std/sync/struct.Mutex.html
pub struct Client {
    tcp_stream: Option<TcpStream>,
    packet_queue: Vec<Packet>,
    listeners: EnumMap<ClientEvent, Vec<ClientListener>>
}

impl Client {

    fn new(tcp_stream: TcpStream) -> Client {
        Client {
            tcp_stream: Some(tcp_stream),
            packet_queue: vec![],
            listeners: enum_map! {
                ClientEvent::Connect    => vec![]
                ClientEvent::Disconnect => vec![]
            }
        }
    }

    #[inline]
    fn is_connected(&self) -> bool {
        self.tcp_stream.is_some()
    }

    async fn connect(&mut self, ip: SocketAddr) {

        self.tcp_stream = Some(TcpStream::connect(ip).await.unwrap());

        for listener in &self.listeners[ClientEvent::Connect] {
            listener(self);
        }
    }

    async fn disconnect(&mut self) {

        self.tcp_stream.as_ref().unwrap().shutdown(Shutdown::Both).unwrap();
        self.tcp_stream = None;

        for listener in &self.listeners[ClientEvent::Disconnect] {
            listener(self);
        }
    }


    fn on_connect(&mut self, listener: fn (&Client)) {
        self.listeners[ClientEvent::Connect].push(listener)
    }

    fn on_disconnect(&mut self, listener: fn (&Client)) {
        self.listeners[ClientEvent::Disconnect].push(listener)
    }


    async fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>> {

        let mut buffer = Vec::with_capacity(size);
        let result = self.tcp_stream.as_ref().unwrap().read_exact(&mut buffer).await;

        if let Err(err) = result {
            self.disconnect();
            return Err(err)
        }

        Ok(buffer)
    }

    async fn read_bytes_as_buffer(&mut self, size: usize) -> Result<ByteBuffer> {
        Ok(ByteBuffer::new(self.read_bytes(size).await?))
    }

    async fn read_bytes_into_buffer(&mut self, buffer: &mut ByteBuffer, amount: usize) -> Result<()> {
        Ok(buffer.write_bytes(&self.read_bytes(amount).await?.as_slice()))
    }


    async fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_bytes(mem::size_of::<i8>()).await?.to_i8())
    }

    async  fn read_i16(&mut self) -> Result<i16> {
        Ok(self.read_bytes(mem::size_of::<i16>()).await?.to_i16())
    }

    async fn read_i32(&mut self) -> Result<i32> {
        Ok(self.read_bytes(mem::size_of::<i32>()).await?.to_i32())
    }

    async fn read_i64(&mut self) -> Result<i64> {
        Ok(self.read_bytes(mem::size_of::<i64>()).await?.to_i64())
    }

    async fn read_i128(&mut self) -> Result<i128> {
        Ok(self.read_bytes(mem::size_of::<i128>()).await?.to_i128())
    }


    async fn read_f32(&mut self) -> Result<f32> {
        Ok(self.read_bytes(mem::size_of::<f32>()).await?.to_f32())
    }

    async fn read_f64(&mut self) -> Result<f64> {
        Ok(self.read_bytes(mem::size_of::<f64>()).await?.to_f64())
    }


    async fn read_u8(&mut self) -> Result<u8> {
        Ok(self.read_bytes(mem::size_of::<u8>()).await?.to_u8())
    }

    async fn read_u16(&mut self) -> Result<u16> {
        Ok(self.read_bytes(mem::size_of::<u16>()).await?.to_u16())
    }

    async fn read_u32(&mut self) -> Result<u32> {
        Ok(self.read_bytes(mem::size_of::<u32>()).await?.to_u32())
    }

    async fn read_u64(&mut self) -> Result<u64> {
        Ok(self.read_bytes(mem::size_of::<u64>()).await?.to_u64())
    }

    async fn read_u128(&mut self) -> Result<u128> {
        Ok(self.read_bytes(mem::size_of::<u128>()).await?.to_u128())
    }


    async fn write(&mut self, packet: Packet) {
        self.packet_queue.push(packet)
    }

    async fn flush(&mut self) {

        let tcp_stream = self.tcp_stream.borrow_mut();

        for packet in self.packet_queue.drain(0..) {
            tcp_stream.as_ref().unwrap().write_all(&packet.write_buffer).await.unwrap();
        }
    }

    async fn write_and_flush(&mut self, packet: Packet) {
        self.write(packet);
        self.flush();
    }

}

