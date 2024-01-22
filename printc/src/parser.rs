const IDENTIFIER_PRINT: &str = "print";

/// Represents different types of nodes in the AST.
pub enum AstNode {
    /// Statement to print text with new line.
    PrintStatement(String),
}

/// Abstract Syntax Tree (AST) that represents the syntactic structure of source code. 
pub struct Ast {
    nodes: Vec<AstNode>,
}

impl Ast {
    /// Parses source code into AST.
    fn from_source(code: &str) -> Ast {
        let mut nodes: Vec<AstNode> = Vec::new();
        for line in code.lines() {
            if line.starts_with(&format!("{} ", IDENTIFIER_PRINT)) {
                let msg = line.trim_start_matches(&format!("{} ", IDENTIFIER_PRINT)).to_string();
                nodes.push(AstNode::PrintStatement(msg));
            }
        }
        Ast { nodes }
    }
}

pub struct Parser {}

impl Parser {
    /// Constructs the AST from source code.
    pub fn construct_ast(code: &str) -> Ast {
        Ast::from_source(code)
    }
}