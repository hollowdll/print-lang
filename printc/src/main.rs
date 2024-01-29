use std::env;
use std::error::Error;
use std::fs::read_to_string;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use printc::parser::AstNode;
use printc::parser::Parser;

type MultiplyFunc = unsafe extern "C" fn(i64, i64) -> i64;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    fn jit_compile_multiply(&self) -> Option<JitFunction<MultiplyFunc>> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into()], false);
        let function = self.module.add_function("multiply", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0)?.into_int_value();
        let y = function.get_nth_param(1)?.into_int_value();

        let result = self.builder.build_int_mul(x, y, "multiply").unwrap();

        self.builder.build_return(Some(&result)).unwrap();

        unsafe { self.execution_engine.get_function("multiply").ok() }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: printc <input_file>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let code = read_to_string(filename).expect("Error reading file");
    let ast = Parser::construct_ast(&code);
    
    let context = Context::create();
    let module = context.create_module("print_lang");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
    let codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
        execution_engine,
    };

    let multiply = codegen
        .jit_compile_multiply()
        .ok_or("Unable to JIT compile `multiply`")?;

    for node in ast.nodes.into_iter() {
        match node {
            AstNode::MultiplyInt64Statement(x, y) => unsafe {
                println!("{} * {} = {}", x, y, multiply.call(x, y));
            },
            AstNode::PrintStatement(msg) => {
                println!("{}", msg)
            }
        }
    }

    Ok(())
}
