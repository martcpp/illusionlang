mod lexericals;
use crate::lexericals::lexer::Lexer;


fn main() {
    // let token = Token {
    //     kind: Tokenkind::Ident,
    //     literal: String::from("example"),
    // };
    // println!("Token kind: {:?}, literal: {}", token.kind, token.literal);
    // println!("Token {:?}", token);

        let input =r#"
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        "#;

    let mut lexer = Lexer::new(input);
    println!("Lexer initialized with input: {}", input);
    let mut tokens = Vec::new();
    let mut token = lexer.next_token();
    while token.kind != lexericals::tokenizer::Tokenkind::Eof {
        tokens.push(token.literal);
        token = lexer.next_token();
    }

    // loop {
    //     let token = lexer.next_token();
    //     if token.kind == lexericals::tokenizer::Tokenkind::Eof {
    //         break;
    //     }
    //     tokens.push(token.literal);
    // }
    println!("Tokens: {:?}", tokens);
     
    // for (idx,token) in lexer.input.clone().into_iter().enumerate() {
    //     println!("charater at index:{} token:{}", idx, token);
    //     let token = lexer.next_token();
    //     println!("Tokenkind at index:{} is:{}", idx, token.kind);
    //     println!("literal at index:{} is:{} ", idx, token.literal);
    // }

    // println!(" last Current character: {:?}", lexer.next_token());
    
}

fn read_string(ch:char) -> String {
    todo!("Implement string reading logic");
}
