mod lexericals;
mod repl;
use std::io;

use crate::lexericals::lexer::Lexer;
use crate::lexericals::tokenizer::{Token, Tokenkind};
use crate::repl::start;

fn main() {
    // Initialize the lexer with the input string
    println!("this is the issionlang terminal \nwrite your code below");
    println!("press ctrl+c to exit the terminal");
    start(io::stdin(), io::stdout());
    //print_tokens();

}


fn print_tokens() {
    let mut lexer = Lexer::new(read_input());
    let runer = expected_token();

    for (idx, token) in runer.into_iter().enumerate() {
        // println!("charater at index:{} token:{:?}", idx, token);
        let inside_token = lexer.next_token();
        println!(
            "Tokenkind at index:{} expected is:{} gotten is:{}",
            idx, token.kind, inside_token.kind
        );
        println!(
            "literal at index:{} expected is:{} gotten is:{}",
            idx, token.literal, inside_token.literal
        );
    }
}


fn read_input() -> &'static str {
    let input = r#"
        let five = 5000000;
        let floattest = 5.0;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        == True; False;
    "#;

    input
}
fn expected_token() -> Vec<Token> {

        let expect_tokens = vec![
            Token::new(Tokenkind::Let, String::from("let")),
            Token::new(Tokenkind::Ident, String::from("five")),
            Token::new(Tokenkind::Assign, String::from("=")),
            Token::new(Tokenkind::Int, String::from("5000000")),
            Token::new(Tokenkind::Semicolon, String::from(";")),
            Token::new(Tokenkind::Let, String::from("let")),
            Token::new(Tokenkind::Ident, String::from("floattest")),
            Token::new(Tokenkind::Assign, String::from("=")),
            Token::new(Tokenkind::Float, String::from("5.0")),
            Token::new(Tokenkind::Semicolon, String::from(";")),
            Token::new(Tokenkind::Let, String::from("let")),
            Token::new(Tokenkind::Ident, String::from("ten")),
            Token::new(Tokenkind::Assign, String::from("=")),
            Token::new(Tokenkind::Int, String::from("10")),
            Token::new(Tokenkind::Semicolon, String::from(";")),
            Token::new(Tokenkind::Let, String::from("let")),
            Token::new(Tokenkind::Ident, String::from("add")),
            Token::new(Tokenkind::Assign, String::from("=")),
            Token::new(Tokenkind::Function, String::from("fn")),
            Token::new(Tokenkind::Lparen, String::from("(")),
            Token::new(Tokenkind::Ident, String::from("x")),
            Token::new(Tokenkind::Comma, String::from(",")),
            Token::new(Tokenkind::Ident, String::from("y")),
            Token::new(Tokenkind::Rparen, String::from(")")),
            Token::new(Tokenkind::Lbrace, String::from("{")),
            Token::new(Tokenkind::Ident, String::from("x")),
            Token::new(Tokenkind::Plus, String::from("+")),
            Token::new(Tokenkind::Ident, String::from("y")),
            Token::new(Tokenkind::Semicolon, String::from(";")),
            Token::new(Tokenkind::Rbrace, String::from("}")),
            Token::new(Tokenkind::Semicolon, String::from(";")),
            Token::new(Tokenkind::Let, String::from("let")),
            Token::new(Tokenkind::Ident, String::from("result")),
            Token::new(Tokenkind::Assign, String::from("=")),
            Token::new(Tokenkind::Ident, String::from("add")),
            Token::new(Tokenkind::Lparen, String::from("(")),
            Token::new(Tokenkind::Ident, String::from("five")),
            Token::new(Tokenkind::Comma, String::from(",")),
            Token::new(Tokenkind::Ident, String::from("ten")),
            Token::new(Tokenkind::Rparen, String::from(")")),
            Token::new(Tokenkind::Semicolon, String::from(";")),
            Token::new(Tokenkind::Eq, String::from("==")),
            Token::new(Tokenkind::Bool, String::from("True")),
            Token::new(Tokenkind::Semicolon, String::from(";")),
            Token::new(Tokenkind::Bool, String::from("False")),
            Token::new(Tokenkind::Semicolon, String::from(";")),
            Token::new(Tokenkind::Eof, String::new()),
        ];
    expect_tokens
}


