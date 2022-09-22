pub mod leb128;
pub mod wasm;

trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}