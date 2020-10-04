use enum_map::{enum_map, Enum, EnumMap};
use async_std::net::{TcpStream, Shutdown};
use crate::net::packet::Packet;
use crate::net::buffer::ByteBuffer;
use crate::extensions::bytes::BytesExt;
use async_std::net::{SocketAddr};
//use async_std::prelude::*;
use async_std::io::Result;
use futures::{AsyncWriteExt, AsyncReadExt};
use async_std::sync::Arc;
use core::mem;
//use futures::{AsyncReadExt, AsyncWriteExt};

type ClientListener = Box<dyn Fn(&Client)>;

#[derive(Debug, Enum)]
pub enum ClientEvent {
    Connect,
    Disconnect
}

// https://docs.rs/async-std/0.99.4/async_std/sync/struct.Mutex.html
pub struct Client {
    pub tcp_stream: Option<TcpStream>,
    pub packet_queue: Vec<Arc<Packet>>,
    pub listeners: EnumMap<ClientEvent, Vec<ClientListener>>
}

impl Default for Client {
    fn default() -> Self {
        Client {
            tcp_stream: None,
            packet_queue: vec![],
            listeners: enum_map! {
                ClientEvent::Connect    => Vec::new(),
                ClientEvent::Disconnect => Vec::new(),
            }
        }
    }
}

impl Client {

    pub fn new(tcp_stream: TcpStream) -> Client {
        Client {
            tcp_stream: Some(tcp_stream),
            packet_queue: vec![],
            listeners: enum_map! {
                ClientEvent::Connect    => Vec::new(),
                ClientEvent::Disconnect => Vec::new(),
            }
        }
    }

    #[inline]
    pub fn is_connected(&self) -> bool {
        self.tcp_stream.is_some()
    }

    pub async fn connect(&mut self, ip: SocketAddr) -> Result<()> {
        
        let tcp_stream = TcpStream::connect(ip).await?;
        tcp_stream.nodelay().unwrap();
        //tcp_stream.
        
        self.tcp_stream = Some(tcp_stream);
        
        
        for listener in &self.listeners[ClientEvent::Connect] {
            listener(self);
        }
        
        Ok(())
    }

    pub async fn disconnect(&mut self) {

        self.tcp_stream.as_ref().as_ref().unwrap().shutdown(Shutdown::Both).unwrap();
        self.tcp_stream = None;

        for listener in &self.listeners[ClientEvent::Disconnect] {
            listener(self);
        }
    }


    pub fn on_connect(&mut self, listener: Box<dyn Fn(&Client)>) {
        self.listeners[ClientEvent::Connect].push(listener)
    }

    pub fn on_disconnect(&mut self, listener: Box<dyn Fn(&Client)>) {
        self.listeners[ClientEvent::Disconnect].push(listener)
    }


    pub async fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>> {

        let mut buffer = vec![0; size];
        let result = self.tcp_stream.as_ref().unwrap().read_exact(&mut buffer).await;

        if let Err(err) = result {
            self.disconnect().await;
            return Err(err)
        }

        Ok(buffer)
    }

    pub async fn read_bytes_as_buffer(&mut self, size: usize) -> Result<ByteBuffer> {
        Ok(ByteBuffer::new(self.read_bytes(size).await?))
    }

    pub async fn read_bytes_into_buffer(&mut self, buffer: &mut ByteBuffer, amount: usize) -> Result<()> {
        buffer.write_bytes(&self.read_bytes(amount).await?.as_slice());
        Ok(())
    }


    pub async fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_bytes(mem::size_of::<i8>()).await?.to_i8())
    }

    pub async fn read_i16(&mut self) -> Result<i16> {
        Ok(self.read_bytes(mem::size_of::<i16>()).await?.to_i16())
    }

    pub async fn read_i32(&mut self) -> Result<i32> {
        Ok(self.read_bytes(mem::size_of::<i32>()).await?.to_i32())
    }

    pub async fn read_i64(&mut self) -> Result<i64> {
        Ok(self.read_bytes(mem::size_of::<i64>()).await?.to_i64())
    }

    pub async fn read_i128(&mut self) -> Result<i128> {
        Ok(self.read_bytes(mem::size_of::<i128>()).await?.to_i128())
    }


    pub async fn read_f32(&mut self) -> Result<f32> {
        Ok(self.read_bytes(mem::size_of::<f32>()).await?.to_f32())
    }

    pub async fn read_f64(&mut self) -> Result<f64> {
        Ok(self.read_bytes(mem::size_of::<f64>()).await?.to_f64())
    }


    pub async fn read_u8(&mut self) -> Result<u8> {
        Ok(self.read_bytes(mem::size_of::<u8>()).await?.to_u8())
    }

    pub async fn read_u16(&mut self) -> Result<u16> {
        Ok(self.read_bytes(mem::size_of::<u16>()).await?.to_u16())
    }

    pub async fn read_u32(&mut self) -> Result<u32> {
        Ok(self.read_bytes(mem::size_of::<u32>()).await?.to_u32())
    }

    pub async fn read_u64(&mut self) -> Result<u64> {
        Ok(self.read_bytes(mem::size_of::<u64>()).await?.to_u64())
    }

    pub async fn read_u128(&mut self) -> Result<u128> {
        Ok(self.read_bytes(mem::size_of::<u128>()).await?.to_u128())
    }


    pub async fn read_utf8(&mut self) -> Result<String> {
        let size = self.read_i16().await.unwrap() as usize;
        Ok(self.read_bytes(size).await?.to_utf8())
    }


    pub async fn write_packet(&mut self, packet: Packet) {
        self.write_packet_arc(Arc::new(packet)).await;
    }

    pub async fn write_packet_arc(&mut self, packet: Arc<Packet>) {
        self.packet_queue.push(packet);
    }

    pub async fn flush(&mut self) {

        let mut tcp_stream = self.tcp_stream.as_ref().unwrap();

        for packet in self.packet_queue.drain(0..) {
            tcp_stream.write_all(&packet.write_buffer).await.unwrap();
        }
    }

    pub async fn write_and_flush_packet(&mut self, packet: Packet) {
        self.write_and_flush_packet_arc(Arc::new(packet)).await;
    }

    pub async fn write_and_flush_packet_arc(&mut self, packet: Arc<Packet>) {
        self.write_packet_arc(packet).await;
        self.flush().await;
    }
}