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

    // let input = "+=(){};";
    // let mut lexer = Lexer::new(input);
    // println!("Lexer initialized with input: {}", input);
    // // println!("Current character: {:?}", lexer);

    // for (idx,token) in lexer.input.clone().into_iter().enumerate() {
    //     println!("Token at index {}: {}", idx, token);
    //     println!("Current character: {:?}", lexer.next_token());
    // }

    // // println!(" last Current character: {:?}", lexer.next_token());
    read_token();

}


fn read_token() {
        let input ="=+(){},;";
        println!("Reading tokens from input: {}", input.len());
        let expect_token = vec![
            Token::new (Tokenkind::Assign,String::from("=")),
            Token::new (Tokenkind::Plus, String::from("+")),
            Token::new (Tokenkind::Lparen, String::from("(")),
            Token::new (Tokenkind::Rparen, String::from(")")),
            Token::new (Tokenkind::Lbrace, String::from("{")),
            Token::new (Tokenkind::Rbrace, String::from("}")),
            Token::new(Tokenkind::Comma, String::from(",")),
            Token::new (Tokenkind::Semicolon,String::from(";")),
            Token::new(Tokenkind::Eof, String::new()), // Eof token
        ];
        println!("Expecting {} tokens", expect_token.len());
        println!("Expecting tokens: {:#?}", expect_token);

        let mut lexer = Lexer::new(input);
        println!("Lexer initialized with input: {:?}", lexer);
        for (idx,exp_token) in expect_token.iter().enumerate() {
            let recv_token = lexer.next_token();
            println!("Reading token at index: {}: expected token: {:?}", idx, exp_token);
            println!("reading token at: {} \n expected token: {}  \n found token: {}",idx,exp_token.kind,recv_token.kind);
            //println!("read literal at: {} \n expected literal: {}  \n found literal: {}",idx,exp_token.literal,recv_token.literal);  // Move to the next character
        }
    }