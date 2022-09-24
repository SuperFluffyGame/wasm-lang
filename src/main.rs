use wasm_lang::{
    compiler::wasm::{
        func::Func,
        types::{I32instr, ValueType, VarInstr},
        Module,
    },
    parser::{Lexer, Parser},
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

    let input = "let a = 2.1;";
    let mut lexer = Lexer::new(input);
    let toks = lexer.lex();

    println!("{:?}", toks);
    println!("{:?}", lexer.errors);
    let parser = Parser::new(&toks);
    // let program = parser.parse();
}
