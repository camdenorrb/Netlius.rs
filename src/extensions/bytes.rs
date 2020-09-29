use std::convert::TryInto;

pub trait BytesExt {

    fn to_i8(&self) -> i8;

    fn to_i16(&self) -> i16;

    fn to_i32(&self) -> i32;

    fn to_i64(&self) -> i64;

    fn to_i128(&self) -> i128;


    fn to_f32(&self) -> f32;

    fn to_f64(&self) -> f64;


    fn to_u8(&self) -> u8;

    fn to_u16(&self) -> u16;

    fn to_u32(&self) -> u32;

    fn to_u64(&self) -> u64;

    fn to_u128(&self) -> u128;


    fn to_utf8(&self) -> String;

}

impl BytesExt for [u8] {

    fn to_i8(&self) -> i8 {
        self[0] as i8
    }

    fn to_i16(&self) -> i16 {
        i16::from_le_bytes(self.try_into().unwrap())
    }

    fn to_i32(&self) -> i32 {
        i32::from_le_bytes(self.try_into().unwrap())
    }

    fn to_i64(&self) -> i64 {
        i64::from_le_bytes(self.try_into().unwrap())
    }

    fn to_i128(&self) -> i128 {
        i128::from_le_bytes(self.try_into().unwrap())
    }

    fn to_f32(&self) -> f32 {
        f32::from_le_bytes(self.try_into().unwrap())
    }

    fn to_f64(&self) -> f64 {
        f64::from_le_bytes(self.try_into().unwrap())
    }

    fn to_u8(&self) -> u8 {
        u8::from_le_bytes(self.try_into().unwrap())
    }

    fn to_u16(&self) -> u16 {
        u16::from_le_bytes(self.try_into().unwrap())
    }

    fn to_u32(&self) -> u32 {
        u32::from_le_bytes(self.try_into().unwrap())
    }

    fn to_u64(&self) -> u64 {
        u64::from_le_bytes(self.try_into().unwrap())
    }

    fn to_u128(&self) -> u128 {
        u128::from_le_bytes(self.try_into().unwrap())
    }

    fn to_utf8(&self) -> String {
        String::from_utf8(self.to_vec()).unwrap()
    }

}

impl BytesExt for Vec<u8> {

    fn to_i8(&self) -> i8 {
        self[0] as i8
    }

    fn to_i16(&self) -> i16 {
        self.as_slice().to_i16()
    }

    fn to_i32(&self) -> i32 {
        self.as_slice().to_i32()
    }

    fn to_i64(&self) -> i64 {
        self.as_slice().to_i64()
    }

    fn to_i128(&self) -> i128 {
        self.as_slice().to_i128()
    }

    fn to_f32(&self) -> f32 {
        self.as_slice().to_f32()
    }

    fn to_f64(&self) -> f64 {
        self.as_slice().to_f64()
    }

    fn to_u8(&self) -> u8 {
        self.as_slice().to_u8()
    }

    fn to_u16(&self) -> u16 {
        self.as_slice().to_u16()
    }

    fn to_u32(&self) -> u32 {
        self.as_slice().to_u32()
    }

    fn to_u64(&self) -> u64 {
        self.as_slice().to_u64()
    }

    fn to_u128(&self) -> u128 {
        self.as_slice().to_u128()
    }

    fn to_utf8(&self) -> String {
        String::from_utf8(self.to_vec()).unwrap()
    }

}