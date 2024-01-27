use std::env;
use std::fs::read_to_string;
use printc::{
    parser::Parser,
    codegen,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let code = read_to_string(filename).expect("Error reading file");

    let ast = Parser::construct_ast(&code);
    let ir = codegen::generate_ir(&ast);

    println!("Generated LLVM IR:\n{}", ir);
}
