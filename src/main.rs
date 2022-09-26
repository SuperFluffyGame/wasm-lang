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

    let input = "1 + 2";
    let mut lexer = Lexer::new(input);

    // loop {
    //     println!("{:?}", lexer.peek());
    //     if let TokenType::EOF = lexer.peek().t {
    //         std::process::exit(1)
    //     }
    //     lexer.next();
    // }

    let mut parser = Parser::new(&mut lexer);
    let program = parser.parse();

    println!("PROGRAM: {:#?}", program);
    println!("PARSE ERRORS: {:?}", parser.errors);
    println!("LEXER ERRORS: {:?}", lexer.errors);
}
