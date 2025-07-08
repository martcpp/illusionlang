mod lexericals;
use crate::lexericals::tokenizer::{Token, Tokenkind};


fn main() {
    let token = Token {
        kind: Tokenkind::Ident,
        literal: String::from("example"),
    };
    println!("Token kind: {:?}, literal: {}", token.kind, token.literal);
}
