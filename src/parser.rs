pub mod token;
pub mod ast;
pub mod lexer;
pub mod syntax_analyzer;


use lexer::Lexer;

pub fn run(content: String) {
    let mut lexer = Lexer::new(&content);
    let tokens = lexer.tokenize();


    for token in tokens { print!("{}", token); }

    // todo: implement the syntax analyzer and return an AST

}
