use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::support::LLVMString;

use crate::parser::Ast;

/// Generates LLVM IR from AST.
pub fn generate_ir(ast: &Ast) -> LLVMString {
    let context = Context::create();
    let module = context.create_module("print-lang");
    let builder = context.create_builder();

    // TODO: arguments
    for node in ast.nodes {
        match node {
            AstNode::PrintStatement(msg) => {
                let function =
                    module.add_function("print", context.i32_type().fn_type(&[], false), None);
                let basic_block = context.append_basic_block(function, "entry");
                builder.position_at_end(basic_block);

                let format_str = context.const_string(message, false);
                let format_str_global =
                    module.add_global(format_str.get_type(), Some(format_str), "str");

                builder.build_return(Some(&context.i32_type().const_int(0, false)));
            }
        }
    }

    module.print_to_string()
}
