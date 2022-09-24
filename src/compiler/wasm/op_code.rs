#[allow(dead_code)]
pub mod section {
    pub const MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];
    pub const VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
    pub const TYPE: u8 = 0x01;
    pub const IMPORT: u8 = 0x02;
    pub const FUNCTION: u8 = 0x03;
    pub const TABLE: u8 = 0x04;
    pub const LINEAR_MEMORY: u8 = 0x05;
    pub const GLOBAL: u8 = 0x06;
    pub const EXPORT: u8 = 0x07;
    pub const START: u8 = 0x08;
    pub const ELEMENT: u8 = 0x09;
    pub const CODE: u8 = 0x0a;
    pub const DATA: u8 = 0x0b;
}

#[allow(dead_code)]
pub mod value {
    pub const I32: u8 = 0x7f;
    pub const I64: u8 = 0x7e;
    pub const F32: u8 = 0x7d;
    pub const F64: u8 = 0x7c;
    pub const V128: u8 = 0x7b;
    pub const FUNCREF: u8 = 0x70;
    pub const ENTERNREF: u8 = 0x6f;
}

pub mod instr {
    pub mod i32 {
        pub const CONST: u8 = 0x41;
    }
}
