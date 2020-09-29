use std::io::Write;

pub struct Packet {
    pub(crate) write_buffer: Vec<u8>,
    pub(crate) is_prepending: bool,
    pub(crate) prepend_write_queue: Vec<u8>
}

impl Packet {

    pub fn new() -> Packet {
        Packet {
            write_buffer: vec![],
            is_prepending: false,
            prepend_write_queue: vec![]
        }
    }


    pub fn i8(self, value: i8) -> Packet {
        self.bytes(&value.to_be_bytes())
    }

    pub fn i16(self, value: i16) -> Packet {
        self.bytes(&value.to_be_bytes())
    }

    pub fn i32(self, value: i32) -> Packet {
        self.bytes(&value.to_be_bytes())
    }

    pub fn i64(self, value: i64) -> Packet {
        self.bytes(&value.to_be_bytes())
    }

    pub fn i128(self, value: i128) -> Packet {
        self.bytes(&value.to_be_bytes())
    }


    pub fn f32(self, value: f32) -> Packet {
        self.bytes(&value.to_be_bytes())
    }

    pub fn f64(self, value: f64) -> Packet {
        self.bytes(&value.to_be_bytes())
    }


    pub fn u8(self, value: u8) -> Packet {
        self.bytes(&value.to_be_bytes())
    }

    pub fn u16(self, value: u16) -> Packet {
        self.bytes(&value.to_be_bytes())
    }

    pub fn u32(self, value: u32) -> Packet {
        self.bytes(&value.to_be_bytes())
    }

    pub fn u64(self, value: u64) -> Packet {
        self.bytes(&value.to_be_bytes())
    }

    pub fn u128(self, value: u128) -> Packet {
        self.bytes(&value.to_be_bytes())
    }


    pub fn utf8(self, string: &str) -> Packet {
        self.bytes(string.as_bytes())
    }


    pub fn bytes(mut self, values: &[u8]) -> Packet {

        if self.is_prepending {
            self.prepend_write_queue.write_all(values).unwrap();
        }
        else {
            self.write_buffer.write_all(values).unwrap();
        }

        self
    }


    #[inline]
    pub fn prepend(mut self, block: fn(packet: &mut Packet)) -> Packet {

        self.is_prepending = true;
        block(&mut self);
        self.is_prepending = false;

        self.write_buffer.splice(0..0, self.prepend_write_queue.drain(0..));

        self
    }

}