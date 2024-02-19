#![allow(unused)]

use std::str::Chars;

/// Represents different kinds of tokens.
#[derive(Debug)]
pub enum TokenKind {
    Literal(Literal),
    Delimiter(Delimiter),
    Identifier(String),
}

/// Tokens represent the smallest units of syntax.
/// Token contains metadata related to it and the token type.
#[derive(Debug)]
pub struct Token {
    /// The line number.
    pub line_num: usize,
    /// The character number starting from the first character of the line.
    pub char_num: usize,
    pub kind: TokenKind,
}

impl Token {
    fn new(line_num: usize, char_num: usize, kind: TokenKind) -> Self {
        Token {
            line_num,
            char_num,
            kind,
        }
    }
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
    input: &'a str,
    current_line_num: usize,
    current_char_num: usize,
    chars: Chars<'a>,
    position: usize,
    next: Option<char>,
}

impl<'a> Lexer<'a> {
    fn advance(&mut self) {
        self.position += 1;
        self.next = self.chars.next();
        self.current_char_num += 1;
    }

    fn new_line(&mut self) {
        self.current_line_num += 1;
        // reset current character to the start of new line
        self.current_char_num = 1;
    }

    fn read_identifier(&mut self, tokens: &mut Vec<Token>) {
        let pos_start = self.position;
        let mut pos_end = pos_start;
        let char_start = self.current_char_num;

        loop {
            self.advance();
            if let Some(ch) = self.next {
                pos_end += 1;
                if ch.is_whitespace() || ch == '(' {
                    tokens.push(Token::new(
                        self.current_line_num,
                        char_start,
                        TokenKind::Identifier(self.input[pos_start..pos_end].to_string()),
                    ));
                    break;
                }
            } else {
                eprintln!("token error: identifier never ended");
                break;
            }
        }
    }

    fn read_string_literal(&mut self, tokens: &mut Vec<Token>) {
        let pos_start = self.position;
        let mut pos_end = pos_start;
        let char_start = self.current_char_num;

        loop {
            self.advance();
            if let Some(ch) = self.next {
                pos_end += 1;
                if ch == '"' {
                    self.advance();
                    let val;

                    if pos_end - pos_start <= 1 {
                        val = "".to_string();
                    } else {
                        val = self.input[pos_start + 1..pos_end].to_string();
                    }

                    tokens.push(Token::new(
                        self.current_line_num,
                        char_start,
                        TokenKind::Literal(Literal::String(val)),
                    ));
                    break;
                }
            } else {
                eprintln!("token error: string literal never ended with \"");
                break;
            }
        }
    }
}

/// Converts source code into lexical tokens.
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    if input.is_empty() {
        return tokens;
    }

    let mut lexer = Lexer {
        input,
        current_line_num: 1,
        current_char_num: 1,
        chars: input[..].chars(),
        position: 0,
        next: None,
    };
    lexer.next = lexer.chars.next();

    loop {
        if let Some(ch) = lexer.next {
            if ch.is_whitespace() {
                lexer.advance();

                if is_newline(ch) {
                    lexer.new_line();
                }

                continue
            }
            match ch {
                ',' => {
                    tokens.push(Token::new(
                        lexer.current_line_num,
                        lexer.current_char_num,
                        TokenKind::Delimiter(Delimiter::Comma),
                    ));
                    lexer.advance();
                }
                ';' => {
                    tokens.push(Token::new(
                        lexer.current_line_num,
                        lexer.current_char_num,
                        TokenKind::Delimiter(Delimiter::Semicolon),
                    ));
                    lexer.advance();
                }
                '(' => {
                    tokens.push(Token::new(
                        lexer.current_line_num,
                        lexer.current_char_num,
                        TokenKind::Delimiter(Delimiter::LeftParen),
                    ));
                    lexer.advance();
                }
                ')' => {
                    tokens.push(Token::new(
                        lexer.current_line_num,
                        lexer.current_char_num,
                        TokenKind::Delimiter(Delimiter::RightParen),
                    ));
                    lexer.advance();
                }
                '"' => {
                    lexer.read_string_literal(&mut tokens);
                }
                _ => {
                    if is_identifier(ch) {
                        lexer.read_identifier(&mut tokens);
                    } else {
                        eprintln!("token error: unknown char: {}", ch);
                        break
                    }
                }
            }
        } else {
            break
        }
    }

    tokens
}

fn is_newline(ch: char) -> bool {
    match ch.to_string().as_str() {
        "\n" => true,
        "\r\n" => true,
        _ => false,
    }
}

fn is_identifier(ch: char) -> bool {
    ch.is_ascii_alphabetic()
}

fn is_integer(ch: char) -> bool {
    ch.is_digit(10)
}

// TODO
fn read_integer() {}
