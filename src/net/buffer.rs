use std::convert::TryInto;
use std::mem;

// TODO: Support variable endianness
pub struct ByteBuffer {
    position: usize,
    limit: usize,
    backend: Vec<u8>
}

impl ByteBuffer {

    pub fn new(backend: Vec<u8>) -> ByteBuffer {
        ByteBuffer {
            position: 0,
            limit: backend.len(),
            backend
        }
    }


    pub fn read_bytes(&mut self, size: usize) -> &[u8] {

        let end_position = self.position + size;
        let data = &self.backend[self.position..end_position];

        self.position = end_position;

        data
    }

    pub fn read_i8(&mut self) -> i8 {

        let data = self.backend[self.position];
        self.position += mem::size_of::<i8>();

        data as i8
    }

    pub fn read_i16(&mut self) -> i16 {
        i16::from_le_bytes(self.read_bytes(mem::size_of::<i16>()).try_into().unwrap())
    }

    pub fn read_i32(&mut self) -> i32 {
        i32::from_le_bytes(self.read_bytes(mem::size_of::<i32>()).try_into().unwrap())
    }

    pub fn read_i64(&mut self) -> i64 {
        i64::from_le_bytes(self.read_bytes(mem::size_of::<i64>()).try_into().unwrap())
    }

    pub fn read_i128(&mut self) -> i128 {
        i128::from_le_bytes(self.read_bytes(mem::size_of::<i128>()).try_into().unwrap())
    }


    pub fn read_f32(&mut self) -> f32 {
        f32::from_le_bytes(self.read_bytes(mem::size_of::<f32>()).try_into().unwrap())
    }

    pub fn read_f64(&mut self) -> f64 {
        f64::from_le_bytes(self.read_bytes(mem::size_of::<f64>()).try_into().unwrap())
    }


    pub fn read_u8(&mut self) -> u8 {

        let data = self.backend[self.position];
        self.position += mem::size_of::<u8>();

        data
    }

    pub fn read_u16(&mut self) -> u16 {
        u16::from_le_bytes(self.read_bytes(mem::size_of::<u16>()).try_into().unwrap())
    }

    pub fn read_u32(&mut self) -> u32 {
        u32::from_le_bytes(self.read_bytes(mem::size_of::<u32>()).try_into().unwrap())
    }

    pub fn read_u64(&mut self) -> u64 {
        u64::from_le_bytes(self.read_bytes(mem::size_of::<u64>()).try_into().unwrap())
    }

    pub fn read_u128(&mut self) -> u128 {
        u128::from_le_bytes(self.read_bytes(mem::size_of::<u128>()).try_into().unwrap())
    }


    pub fn read_utf8(&mut self) -> String {
        let size = self.read_i16();
        String::from_utf8(self.read_bytes(size as usize).to_vec()).unwrap()
    }


    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.backend[self.position] = *byte;
            self.position += 1;
        }
    }

    pub fn write_i8(&mut self, value: i8) {
        self.backend[self.position] = value as u8;
        self.position += 1;
    }

    pub fn write_i16(&mut self, value: i16) {
        self.write_bytes(&value.to_le_bytes());
    }

    pub fn write_i32(&mut self, value: i32) {
        self.write_bytes(&value.to_le_bytes());
    }

    pub fn write_i64(&mut self, value: i64) {
        self.write_bytes(&value.to_le_bytes());
    }

    pub fn write_i128(&mut self, value: i128) {
        self.write_bytes(&value.to_le_bytes());
    }


    pub fn write_f32(&mut self, value: f32) {
        self.write_bytes(&value.to_le_bytes());
    }

    pub fn write_f64(&mut self, value: f64) {
        self.write_bytes(&value.to_le_bytes());
    }


    pub fn write_u8(&mut self, value: u8) {
        self.backend[self.position] = value;
        self.position += 1;
    }

    pub fn write_u16(&mut self, value: u16) {
        self.write_bytes(&value.to_le_bytes());
    }

    pub fn write_u32(&mut self, value: u32) {
        self.write_bytes(&value.to_le_bytes());
    }

    pub fn write_u64(&mut self, value: u64) {
        self.write_bytes(&value.to_le_bytes());
    }

    pub fn write_u128(&mut self, value: u128) {
        self.write_bytes(&value.to_le_bytes());
    }


    pub fn write_utf8(&mut self, value: String) {
        self.write_i32(value.len() as i32);
        self.write_bytes(value.as_bytes());
    }


    pub fn flip(&mut self) {
        self.limit = self.position;
        self.position = 0;
    }

    pub fn clear(&mut self) {
        self.limit = self.backend.len();
        self.position = 0;
    }

}