use crate::wasm::op_code::value;
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
    V128,
}
impl ValueType {
    pub fn byte(&self) -> u8 {
        match self {
            Self::I32 => value::I32,
            Self::I64 => value::I64,
            Self::F32 => value::F32,
            Self::F64 => value::F64,
            Self::V128 => value::V128,
        }
    }
}
pub enum I32instr {
    Const(i32),
    Add,
}
impl Into<Instr> for I32instr {
    fn into(self) -> Instr {
        Instr::I32(self)
    }
}
pub enum VarInstr {
    LocalGet(u32),
}
impl Into<Instr> for VarInstr {
    fn into(self) -> Instr {
        Instr::Var(self)
    }
}
pub enum Instr {
    I32(I32instr),
    Var(VarInstr),
}
// impl From<I32instr> for Instr {
//     fn from(i: I32instr) -> Self {
//         Self::I32(i)
//     }
// }
// impl From<VarInstr> for Instr {
//     fn from(i: VarInstr) -> Self {
//         Self::Var(i)
//     }
// }
