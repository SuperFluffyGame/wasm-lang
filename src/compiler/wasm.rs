pub mod func;
pub mod op_code;
pub mod types;
use func::Func;

use super::leb128::leb_u32;

pub struct Module<'a> {
    functions: Vec<&'a Func>,
}
impl<'a> Module<'a> {
    pub fn new() -> Self {
        Module {
            functions: Vec::new(),
        }
    }
    pub fn add_func(&mut self, f: &'a Func) {
        self.functions.push(f);
    }

    pub fn export_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(op_code::section::MAGIC);
        bytes.extend(op_code::section::VERSION);

        // type section
        {
            let mut type_section = Vec::<u8>::new();
            type_section.push(op_code::section::TYPE);

            //  type data
            let mut type_section_vec = Vec::new();
            // number of types
            type_section_vec.extend(leb_u32(self.functions.len() as u64));
            for f in &self.functions {
                let func_type_bytes = f.type_section_bytes();
                type_section_vec.extend(func_type_bytes);
            }
            // section length in bytes
            type_section.extend(leb_u32(type_section_vec.len() as u64));
            // type data
            type_section.extend(type_section_vec);
            bytes.extend(type_section);
        }
        // function section
        {
            let mut function_section = Vec::<u8>::new();
            function_section.push(op_code::section::FUNCTION);
            // function data
            let mut function_section_vec = Vec::new();
            // number of functions
            function_section_vec.extend(leb_u32(self.functions.len() as u64));
            let mut i = 0;
            for _ in &self.functions {
                function_section_vec.extend(leb_u32(i));
                i += 1;
            }
            function_section.extend(leb_u32(function_section_vec.len() as u64));
            function_section.extend(function_section_vec);
            bytes.extend(function_section);
        }
        bytes
    }
}
