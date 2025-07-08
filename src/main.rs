mod lexericals;
use crate::lexericals::tokenizer::{Token, Tokenkind};
use crate::lexericals::lexer::Lexer;


fn main() {
    // let token = Token {
    //     kind: Tokenkind::Ident,
    //     literal: String::from("example"),
    // };
    // println!("Token kind: {:?}, literal: {}", token.kind, token.literal);
    // println!("Token {:?}", token);

    let input = "=+(){};";
    let lexer = Lexer::new(input);

    for (idx,token) in lexer.input.iter().enumerate() {
        println!("Token at index {}: {}", idx, token);
        println!("Current character: {:?}", lexer);
    }

    println!(" last Current character: {:?}", lexer);

    
}