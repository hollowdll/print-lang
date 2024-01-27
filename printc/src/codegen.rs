use inkwell::context::Context;
use inkwell::support::LLVMString;
use crate::parser::{
    Ast,
    AstNode,
};

const MAIN_FN_NAME: &str = "main";
const PRINT_FN_NAME: &str = "printf";

/// Generates LLVM IR from AST.
pub fn generate_ir(ast: &Ast) -> LLVMString {
    let context = Context::create();
    let module = context.create_module("print-lang");
    let builder = context.create_builder();

    for node in ast.nodes.iter() {
        match node {
            AstNode::PrintStatement(msg) => {
                // Declare the printf function signature
                let i8_ptr_type = context.i8_type().ptr_type(inkwell::AddressSpace::default());
                let printf_type = context.i32_type().fn_type(&[i8_ptr_type.into()], true);
                let printf_func = module.add_function(PRINT_FN_NAME, printf_type, None);

                // Declare the format string constant
                let format_str = context.const_string(format!("{}\n", msg).as_bytes(), false);
                let format_str_global = module.add_global(format_str.get_type(), None, "format_string");
                format_str_global.set_initializer(&format_str);
                let format_str_ptr = builder.build_pointer_cast(
                    format_str_global.as_pointer_value(),
                    i8_ptr_type,
                    "format_string_ptr",
                ).unwrap();

                // Create the main function
                let main_func = module.add_function(MAIN_FN_NAME, context.i32_type().fn_type(&[], false), None);
                let basic_block = context.append_basic_block(main_func, "entry");

                // Insert code to call printf with the format string
                builder.position_at_end(basic_block);
                let _ = builder.build_call(printf_func, &[format_str_ptr.into()], PRINT_FN_NAME);

                // Return 0
                let return_value = context.i32_type().const_int(0, false);
                builder.build_return(Some(&return_value)).unwrap();
            }
        }
    }

    module.print_to_string()
}
