mod lexericals;
use crate::lexericals::lexer::Lexer;
use crate::lexericals::tokenizer::{Token, Tokenkind};

fn main() {
    // let token = Token {
    //     kind: Tokenkind::Ident,
    //     literal: String::from("example"),
    // };
    // println!("Token kind: {:?}, literal: {}", token.kind, token.literal);
    // println!("Token {:?}", token);

    let input = r#"
        let five = 5000000;
        let floattest = 5.0;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        "#;

    let mut lexer = Lexer::new(input);
    let runer = read();
    //println!("Lexer initialized with input: {}", input);

    // let mut tokens = Vec::new();
    // // let mut token = lexer.next_token();
    // // while token.kind != lexericals::tokenizer::Tokenkind::Eof {
    // //     tokens.push(token.literal);
    // //     token = lexer.next_token();
    // // }

    // loop {
    //     let token = lexer.next_token();
    //     if token.kind == lexericals::tokenizer::Tokenkind::Eof {
    //         break;
    //     }
    //     tokens.push(token.literal);
    // }
    // println!("Tokens: {:?}", tokens);
    // println!("Tokens: {:?}", lexer);

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

    // println!(" last Current character: {:?}", lexer.next_token());
}

// fn read_string(ch:char) -> String {
//     todo!("Implement string reading logic");
// }

fn read() -> Vec<Token> {
    let expected_token = vec![
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
        // Token::new(Tokenkind::Let, String::from("let")),
        // Token::new(Tokenkind::Ident, String::from("ten")),
        // Token::new(Tokenkind::Assign, String::from("=")),
        // Token::new(Tokenkind::Int, String::from("10")),
        // Token::new(Tokenkind::Semicolon, String::from(";")),
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
        Token::new(Tokenkind::Eof, String::new()),
    ];
    expected_token
}

// fn main() {
//     let input = r#"
//         let five = 5;
//         let ten = 10;
//         let add = fn(x, y) {
//             x + y;
//         };
//         let result = add(five, ten);
//     "#;

//     let mut lexer = Lexer::new(input);
//     println!("Lexer initialized with input: {}", input);

//     let mut tokens = Vec::new();

//     loop {
//         let token = lexer.next_token();
//         if token.kind == lexericals::tokenizer::Tokenkind::Eof {
//             break;
//         }
//         tokens.push(token); // push the whole token (no clone)
//     }

//     for (idx, token) in tokens.into_iter().enumerate() {
//         println!("character at index:{} token:{:?}", idx, token);
//         println!("Tokenkind at index:{} is:{:?}", idx, token.kind);
//         println!("literal at index:{} is:{} ", idx, token.literal);
//     }
// }
