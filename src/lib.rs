pub mod compiler;
pub mod parser;

trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}
