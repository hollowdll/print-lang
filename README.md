# print-lang

This is an experiment to make a small custom language with a [lexer](https://en.wikipedia.org/wiki/Lexical_analysis), [parser](https://en.wikipedia.org/wiki/Parsing), and compiler. The compiler uses LLVM and is [Just-In-Time](https://en.wikipedia.org/wiki/Just-in-time_compilation) (JIT). 

The lexer takes the source code and converts it into lexical tokens. The tokens are then passed to the parser which parses the tokens into an Abstract Syntax Tree (AST). The AST represents the syntactic structure of the source code. The AST is then processed statement by statement. Finally, the output of statements are printed to the standard output.

If there is a compilation error, it is printed with the line and character number of the source code where the error occured.

Source code samples are in `input/` directory. The samples have file extension `.print`. However, you can put whatever file extension you want.

Currently there is only one working statement and it does not use LLVM.

**Note!** This project is for learning purposes.

# Requirements

- LLVM version 17

**Note!** You need to have LLVM installed to use the compiler. Most systems require you to compile it from source code because not all needed tools are included in the precompiled installations.

Check these links for more information about how to compile and setup LLVM:
- https://llvm.org/docs/GettingStarted.html
- https://crates.io/crates/llvm-sys

You may need to use additional resources to compile LLVM based on your system. This documentation does not cover them as compiling LLVM from source can be a complex and time consuming task.

Currently used LLVM major version: 17. Only this major version is compatible with this project.

Follow the documentation of these crates carefully:
- https://crates.io/crates/inkwell
- https://crates.io/crates/llvm-sys

# Syntax

`println` statement prints a string to the standard output with a new line.
```
println("Something");
```
Output:
```
Something
```

# Usage

```
cd printc
cargo run -- ../input/helloworld.print
```
OR if you have built the binary with `cargo build`
```
printc ../input/helloworld.print
```
You need to pass a path to the file containing the source code.

# Examples

The output of using source code sample `input/helloworld.print`:
```
Input:
"println(\"Hello world!\");"

Tokens:
[Token { line_num: 1, char_num: 1, kind: Identifier("println") }, Token { line_num: 1, char_num: 8, kind: Delimiter(LeftParen) }, Token { line_num: 1, char_num: 9, kind: Literal(String("Hello world!")) }, Token { line_num: 1, char_num: 23, kind: Delimiter(RightParen) }, Token { line_num: 1, char_num: 24, kind: Delimiter(Semicolon) }]

AST:
Ast { nodes: [StatementPrintln("Hello world!")] }

Hello world!
```

The output of using source code sample `input/multi_line.print`:
```
Input:
"println(\"Hello, world!\");\r\nprintln(\"println\");\r\nprintln(\"foo bar baz\");"

Tokens:
[Token { line_num: 1, char_num: 1, kind: Identifier("println") }, Token { line_num: 1, char_num: 8, kind: Delimiter(LeftParen) }, Token { line_num: 1, char_num: 9, kind: Literal(String("Hello, world!")) }, Token { line_num: 1, char_num: 24, kind: Delimiter(RightParen) }, Token { line_num: 1, char_num: 25, kind: Delimiter(Semicolon) }, Token { line_num: 2, char_num: 1, kind: Identifier("println") }, Token { line_num: 2, char_num: 8, kind: Delimiter(LeftParen) }, Token { line_num: 2, char_num: 9, kind: Literal(String("println")) }, Token { line_num: 2, char_num: 18, kind: Delimiter(RightParen) }, Token { line_num: 2, char_num: 19, kind: Delimiter(Semicolon) }, Token { line_num: 3, char_num: 1, kind: Identifier("println") }, Token { line_num: 3, char_num: 8, kind: Delimiter(LeftParen) }, Token { line_num: 3, char_num: 9, kind: Literal(String("foo bar baz")) }, Token { line_num: 3, char_num: 22, kind: Delimiter(RightParen) }, Token { line_num: 3, char_num: 23, kind: Delimiter(Semicolon) }]

AST:
Ast { nodes: [StatementPrintln("Hello, world!"), StatementPrintln("println"), StatementPrintln("foo bar baz")] }

Hello, world!
println
foo bar baz
```

The output of using source code sample `input/syntax_error.print`:
```
Input:
"println(\"Hello, world!\");\r\nprintln(\"this line has error\";\r\nprintln(\"asd\");"

Tokens:
[Token { line_num: 1, char_num: 1, kind: Identifier("println") }, Token { line_num: 1, char_num: 8, kind: Delimiter(LeftParen) }, Token { line_num: 1, char_num: 9, kind: Literal(String("Hello, world!")) }, Token { line_num: 1, char_num: 24, kind: Delimiter(RightParen) }, Token { line_num: 1, char_num: 25, kind: Delimiter(Semicolon) }, Token { line_num: 2, char_num: 1, kind: Identifier("println") }, Token { line_num: 2, char_num: 8, kind: Delimiter(LeftParen) }, Token { line_num: 2, char_num: 9, kind: Literal(String("this line has error")) }, Token { line_num: 2, char_num: 30, kind: Delimiter(Semicolon) }, Token { line_num: 3, char_num: 1, kind: Identifier("println") }, Token { line_num: 3, char_num: 8, kind: Delimiter(LeftParen) }, Token { line_num: 3, char_num: 9, kind: Literal(String("asd")) }, Token { line_num: 3, char_num: 14, kind: Delimiter(RightParen) }, Token { line_num: 3, char_num: 15, kind: Delimiter(Semicolon) }]

syntax error: expected symbol ')' at line 2 char 30
```