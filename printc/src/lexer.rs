use std::str::Chars;

/// Tokens represent the smallest units of syntax.
/// Tokens are grouped to categories.
#[derive(Debug)]
pub enum Token {
    Literal(Literal),
    Delimiter(Delimiter),
    Identifier(String),
}

#[derive(Debug)]
pub enum Delimiter {
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Integer(i64),
}

/// Lexer analyzes source code and converts it into lexical tokens.
/// Parser then takes the tokens and parses them into AST.
/// 
/// Converting source code into tokens makes the parser's job easier.
pub struct Lexer<'a> {
    chars: Chars<'a>,
    position: usize,
    next: Option<char>,
}

impl<'a> Lexer<'a> {
    fn advance(&mut self) {
        self.position += 1;
        self.next = self.chars.next();
    }
}

/// Converts source code into lexical tokens.
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    if input.is_empty() {
        return tokens;
    }

    let mut lexer = Lexer {
        chars: input[..].chars(),
        position: 0,
        next: None,
    };
    lexer.next = lexer.chars.next();

    loop {
        if let Some(ch) = lexer.next {
            if ch.is_whitespace() {
                continue
            }
            match ch {
                ',' => {
                    tokens.push(Token::Delimiter(Delimiter::Comma));
                    lexer.advance();
                },
                ';' => {
                    tokens.push(Token::Delimiter(Delimiter::Semicolon));
                    lexer.advance();
                },
                '(' => {
                    tokens.push(Token::Delimiter(Delimiter::LeftParen));
                    lexer.advance();
                },
                ')' => {
                    tokens.push(Token::Delimiter(Delimiter::RightParen));
                    lexer.advance();
                },
                '"' => {
                    let pos_start = lexer.position;
                    let mut pos_end = pos_start;
        
                    loop {
                        lexer.advance();
                        if let Some(ch) = lexer.next {
                            pos_end += 1;
                            if ch == '"' {
                                lexer.advance();
                                if pos_end - pos_start <= 1 {
                                    tokens.push(Token::Literal(Literal::String("".to_string())));
                                    break
                                } else {
                                    tokens.push(Token::Literal(Literal::String(input[pos_start+1..pos_end].to_string())));
                                    break
                                }
                            }
                        } else {
                            eprintln!("token error: string literal never ended with \"");
                            break
                        }
                    }
                },
                _ => {
                    if is_identifier(ch) {
                        let pos_start = lexer.position;
                        let mut pos_end = pos_start;
            
                        loop {
                            lexer.advance();
                            if let Some(ch) = lexer.next {
                                pos_end += 1;
                                if ch.is_whitespace() || ch == '(' {
                                    tokens.push(Token::Identifier(input[pos_start..pos_end].to_string()));
                                    break
                                }
                            } else {
                                break
                            }
                        }
                    } else {
                        eprintln!("token error: unknown char: {}", ch);
                    }
                },
            }
        } else {
            break
        }
    }

    // Output tokens
    println!("Tokens: {:?}", tokens);

    tokens
}

fn is_identifier(ch: char) -> bool {
    ch.is_ascii_alphabetic()
}

fn is_integer(ch: char) -> bool {
    ch.is_digit(10)
}

fn read_identifier() {

}

fn read_integer() {

}

fn read_string_literal() {

}