use crate::lexer::{Delimiter, Literal, Token, TokenKind};
use std::{
    io::{Error, ErrorKind}, slice::Iter
};

const IDENTIFIER_PRINTLN: &str = "println";
const IDENTIFIER_MULTIPLY_INT64: &str = "mul_int64";

/// Represents different types of nodes in the AST.
#[derive(Debug)]
pub enum AstNode {
    /// Statement to print text with new line.
    PrintStatement(String),
    /// Statement to multiply two i64 integers.
    MultiplyInt64Statement(i64, i64),
    /// Statement to print text with new line.
    StatementPrintln(String),
}

/// Abstract Syntax Tree (AST) that represents the syntactic structure of source code.
#[derive(Debug)]
pub struct Ast {
    pub nodes: Vec<AstNode>,
}

impl Ast {
    fn new(nodes: Vec<AstNode>) -> Self {
        Ast { nodes }
    }
}

// OLD
impl Ast {
    /// Parses source code into AST.
    fn from_source(code: &str) -> Ast {
        let mut nodes: Vec<AstNode> = Vec::new();
        for line in code.lines() {
            if line.starts_with(&format!("{} ", IDENTIFIER_PRINTLN)) {
                let msg = line
                    .trim_start_matches(&format!("{} ", IDENTIFIER_PRINTLN))
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

/// Parser for parsing tokens into AST.
pub struct Parser<'a> {
    tokens: Iter<'a, Token>,
    current_token: Option<&'a Token>,
}

impl<'a> Parser<'a> {
    /// Constructs the AST from source code.
    pub fn construct_ast(code: &str) -> Ast {
        Ast::from_source(code)
    }

    fn advance(&mut self) {
        self.current_token = self.tokens.next();
    }
}

/// Parses tokens received from lexer into AST.
pub fn parse_tokens(tokens: Vec<Token>) -> Result<Ast, Error> {
    let mut nodes: Vec<AstNode> = Vec::new();

    if tokens.is_empty() {
        return Ok(Ast::new(nodes));
    }

    let mut parser = Parser {
        tokens: tokens.iter(),
        current_token: None,
    };

    // Handle tokens one by one
    loop {
        parser.advance();
        if let Some(token) = parser.current_token {
            match &token.kind {
                TokenKind::Identifier(ident) => match ident.as_str() {
                    IDENTIFIER_PRINTLN => {
                        let arg;
                        parser.advance();
                        if let Some(token) = parser.current_token {
                            match token.kind {
                                TokenKind::Delimiter(Delimiter::LeftParen) => (),
                                _ => {
                                    return Err(Error::new(
                                        ErrorKind::InvalidInput,
                                        format!(
                                            "expected symbol '(' at line {} char {}",
                                            token.line_num, token.char_num
                                        ),
                                    ))
                                }
                            }
                        } else {
                            return Err(Error::new(
                                ErrorKind::InvalidInput,
                                format!(
                                    "expected symbol '(' at the end of line {}",
                                    token.line_num
                                ),
                            ));
                        }

                        parser.advance();
                        if let Some(token) = parser.current_token {
                            match &token.kind {
                                TokenKind::Literal(Literal::String(val)) => {
                                    arg = val;
                                }
                                _ => {
                                    return Err(Error::new(
                                        ErrorKind::InvalidInput,
                                        format!(
                                            "expected string literal at line {} char {}",
                                            token.line_num, token.char_num
                                        ),
                                    ))
                                }
                            }
                        } else {
                            return Err(Error::new(
                                ErrorKind::InvalidInput,
                                format!(
                                    "expected string literal at the end of line {}",
                                    token.line_num
                                ),
                            ));
                        }

                        parser.advance();
                        if let Some(token) = parser.current_token {
                            match token.kind {
                                TokenKind::Delimiter(Delimiter::RightParen) => (),
                                _ => {
                                    return Err(Error::new(
                                        ErrorKind::InvalidInput,
                                        format!(
                                            "expected symbol ')' at line {} char {}",
                                            token.line_num, token.char_num
                                        ),
                                    ))
                                }
                            }
                        } else {
                            return Err(Error::new(
                                ErrorKind::InvalidInput,
                                format!(
                                    "expected symbol ')' at the end of line {}",
                                    token.line_num
                                ),
                            ));
                        }

                        parser.advance();
                        if let Some(token) = parser.current_token {
                            match token.kind {
                                TokenKind::Delimiter(Delimiter::Semicolon) => (),
                                _ => {
                                    return Err(Error::new(
                                        ErrorKind::InvalidInput,
                                        format!(
                                            "expected symbol ';' at line {} char {}",
                                            token.line_num, token.char_num
                                        ),
                                    ))
                                }
                            }
                        } else {
                            return Err(Error::new(
                                ErrorKind::InvalidInput,
                                format!(
                                    "expected symbol ';' at the end of line {}",
                                    token.line_num
                                ),
                            ));
                        }

                        nodes.push(AstNode::StatementPrintln(arg.to_owned()));
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::InvalidInput,
                            format!(
                                "unknown identifier '{}' at line {} char {}",
                                ident, token.line_num, token.char_num
                            ),
                        ))
                    }
                },
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!(
                            "unknown token at line {} char {}",
                            token.line_num, token.char_num
                        ),
                    ))
                }
            }
        } else {
            break;
        }
    }

    Ok(Ast::new(nodes))
}
