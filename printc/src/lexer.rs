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
pub struct Lexer {
    position: usize,
}

/// Converts source code into lexical tokens.
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    if input.is_empty() {
        return tokens;
    }

    let mut lexer = Lexer {
        position: 0,
    };
    let mut chars = input[..].chars();

    loop {
        let next = chars.next();
        if let Some(ch) = next {
            if ch.is_whitespace() {
                continue
            }
            match ch {
                ',' => tokens.push(Token::Delimiter(Delimiter::Comma)),
                ';' => tokens.push(Token::Delimiter(Delimiter::Semicolon)),
                '"' => {
                    let pos_start = lexer.position;
                    let mut pos_end = pos_start;
        
                    loop {
                        let next = chars.next();
                        if let Some(ch) = next {
                            pos_end += 1;
                            if ch == '"' {
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
                '(' => tokens.push(Token::Delimiter(Delimiter::LeftParen)),
                ')' => tokens.push(Token::Delimiter(Delimiter::RightParen)),
                _ => eprintln!("token error: unknown char: {}", ch),
            }
            lexer.position += 1;
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