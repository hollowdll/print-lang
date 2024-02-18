use std::{slice::Iter, vec::IntoIter};

use crate::lexer::{Delimiter, Lexer, Token};

const IDENTIFIER_PRINT: &str = "println";
const IDENTIFIER_MULTIPLY_INT64: &str = "mul_int64";

/// Represents different types of nodes in the AST.
pub enum AstNode {
    /// Statement to print text with new line.
    PrintStatement(String),
    /// Statement to multiply two i64 integers.
    MultiplyInt64Statement(i64, i64),
    /// Statement to print text with new line.
    StatementPrintln(String),
}

/// Abstract Syntax Tree (AST) that represents the syntactic structure of source code.
pub struct Ast {
    pub nodes: Vec<AstNode>,
}

impl Ast {
    fn new(nodes: Vec<AstNode>) -> Self {
        Ast { nodes }
    }
}

impl Ast {
    /// Parses source code into AST.
    fn from_source(code: &str) -> Ast {
        let mut nodes: Vec<AstNode> = Vec::new();
        for line in code.lines() {
            if line.starts_with(&format!("{} ", IDENTIFIER_PRINT)) {
                let msg = line
                    .trim_start_matches(&format!("{} ", IDENTIFIER_PRINT))
                    .to_string();
                nodes.push(AstNode::PrintStatement(msg));
            } else if line.starts_with(&format!("{} ", IDENTIFIER_MULTIPLY_INT64)) {
                let nums_str = line
                    .trim_start_matches(&format!("{} ", IDENTIFIER_MULTIPLY_INT64))
                    .to_string();
                let nums_str: Vec<&str> = nums_str.split(" ").collect();
                let nums: Vec<i64> = nums_str.iter().map(|x| x.parse::<i64>().unwrap()).collect();
                nodes.push(AstNode::MultiplyInt64Statement(
                    nums.get(0).unwrap().clone(),
                    nums.get(1).unwrap().clone(),
                ));
            }
        }
        Ast { nodes }
    }
}

pub struct Parser {
    tokens: IntoIter<Token>,
    current_token: Option<Token>,
}

impl Parser {
    /// Constructs the AST from source code.
    pub fn construct_ast(code: &str) -> Ast {
        Ast::from_source(code)
    }

    fn advance(&mut self) {
        self.current_token = self.tokens.next();
    }
}

/// Parses tokens received from lexer into AST.
pub fn parse_tokens(tokens: Vec<Token>) -> Ast {
    let mut nodes: Vec<AstNode> = Vec::new();

    if tokens.is_empty() {
        return Ast::new(nodes);
    }

    let mut parser = Parser {
        tokens: tokens.into_iter(),
        current_token: None,
    };
    parser.current_token = parser.tokens.next();

    // TODO
    loop {

    }

    Ast::new(nodes)
}
