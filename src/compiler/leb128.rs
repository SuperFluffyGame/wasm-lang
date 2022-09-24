#![allow(dead_code)]

fn to_lebs(n: i64) -> Vec<u8> {
    let mut b = Vec::new();
    leb128::write::signed(&mut b, n).expect("Error Converting to LEB128");
    b
}
fn to_lebu(n: u64) -> Vec<u8> {
    let mut b = Vec::new();
    leb128::write::unsigned(&mut b, n).expect("Error Converting to LEB128");
    b
}

pub fn leb_u32(n: u64) -> Vec<u8> {
    let bytes = to_lebu(n);
    if bytes.len() > 4 {
        panic!("Byte length greater than allowed (Conversion of u64 to LEB128 u32)");
    }
    bytes
}
