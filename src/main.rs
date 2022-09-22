use wasm_lang::wasm::{
    func::Func,
    types::{I32instr, ValueType, VarInstr},
    Module,
};
fn main() {
    println!("Hello, world!");

    let mut module = Module::new();
    let mut add = Func::new(
        "add".to_string(),
        vec![ValueType::I32, ValueType::I32],
        vec![ValueType::I32],
        None,
    );
    add.add_instrs(vec![
        VarInstr::LocalGet(0).into(),
        VarInstr::LocalGet(1).into(),
        I32instr::Add.into(),
    ]);
    module.add_func(&add);

    println!("{:02X?}", module.export_bytes());
}