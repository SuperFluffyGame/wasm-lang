#[allow(dead_code)]
pub mod section {
    const MAGIC: [u8; 4] = [0x00, 0x61, 0x73, 0x6D];
    const VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
    const TYPE: u8 = 0x01;
    const IMPORT: u8 = 0x02;
    const FUNCTION: u8 = 0x03;
    const TABLE: u8 = 0x04;
    const LINEAR_MEMORY: u8 = 0x05;
    const GLOBAL: u8 = 0x06;
    const EXPORT: u8 = 0x07;
    const START: u8 = 0x08;
    const ELEMENT: u8 = 0x09;
    const CODE: u8 = 0x0a;
    const DATA: u8 = 0x0b;
}

#[allow(dead_code)]
pub mod value {
    const I32: u8 = 0x7f;
    const I64: u8 = 0x7e;
    const F32: u8 = 0x7d;
    const F64: u8 = 0x7c;
    const V128: u8 = 0x7b;
    const FUNCREF: u8 = 0x70;
    const ENTERNREF: u8 = 0x6f;
}

pub mod instr {
    pub mod I32 {
        const STOR
    }
}
