use crate::{leb128::leb_u32, wasm::types::*};

pub struct Func {
    name: String,
    params: Vec<ValueType>,
    result: Vec<ValueType>,
    instrs: Vec<Instr>,
}
impl Func {
    pub fn new(
        name: String,
        params: Vec<ValueType>,
        result: Vec<ValueType>,
        instrs: Option<Vec<Instr>>,
    ) -> Self {
        let mut instrs_out = Vec::new();
        if let Some(i) = instrs {
            instrs_out = i;
        }
        Func {
            name,
            params,
            result,
            instrs: instrs_out,
        }
    }
    pub fn add_instr(&mut self, i: Instr) {
        self.instrs.push(i);
    }
    pub fn add_instrs(&mut self, instrs: Vec<Instr>) {
        self.instrs.extend(instrs);
    }

    pub fn type_section_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(0x60);

        let mut param_bytes = Vec::<u8>::new();
        for param in &self.params {
            param_bytes.push(param.byte());
        }
        let param_bytes_len_bytes = leb_u32(param_bytes.len() as u64);

        bytes.extend(param_bytes_len_bytes);
        bytes.extend(param_bytes);

        let mut result_bytes = Vec::<u8>::new();
        for result in &self.result {
            result_bytes.push(result.byte());
        }
        let result_bytes_len_bytes = leb_u32(result_bytes.len() as u64);

        bytes.extend(result_bytes_len_bytes);
        bytes.extend(result_bytes);

        bytes
    }
}
