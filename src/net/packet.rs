use std::io::Write;

pub struct Packet {
    pub(crate) write_buffer: Vec<u8>,
    pub(crate) is_prepending: bool,
    pub(crate) prepend_write_queue: Vec<u8>
}

impl Packet {

    fn new() -> Packet {
        Packet {
            write_buffer: vec![],
            is_prepending: false,
            prepend_write_queue: vec![]
        }
    }


    fn i8(&mut self, value: i8) {
        self.bytes(&value.to_le_bytes())
    }

    fn i16(&mut self, value: i16) {
        self.bytes(&value.to_le_bytes())
    }

    fn i32(&mut self, value: i32) {
        self.bytes(&value.to_le_bytes())
    }

    fn i64(&mut self, value: i64) {
        self.bytes(&value.to_le_bytes())
    }

    fn i128(&mut self, value: i128) {
        self.bytes(&value.to_le_bytes())
    }


    fn f32(&mut self, value: f32) {
        self.bytes(&value.to_le_bytes())
    }

    fn f64(&mut self, value: f64) {
        self.bytes(&value.to_le_bytes())
    }


    fn u8(&mut self, value: u8) {
        self.bytes(&value.to_le_bytes())
    }

    fn u16(&mut self, value: u16) {
        self.bytes(&value.to_le_bytes())
    }

    fn u32(&mut self, value: u32) {
        self.bytes(&value.to_le_bytes())
    }

    fn u64(&mut self, value: u64) {
        self.bytes(&value.to_le_bytes())
    }

    fn u128(&mut self, value: u128) {
        self.bytes(&value.to_le_bytes())
    }


    fn utf8(&mut self, string: &str) {
        self.bytes(string.as_bytes())
    }


    fn bytes(&mut self, values: &[u8]) {
        if self.is_prepending {
            self.prepend_write_queue.write_all(values).unwrap();
        }
        else {
            self.write_buffer.write_all(values).unwrap();
        }
    }


    #[inline]
    fn prepend(&mut self, block: fn(packet: &mut Packet)) {

        self.is_prepending = true;
        block(self);
        self.is_prepending = false;

        self.write_buffer.splice(0..0, self.prepend_write_queue.drain(0..));
    }

}