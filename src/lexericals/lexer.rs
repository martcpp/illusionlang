use super::tokenizer::{Token,Tokenkind};

pub struct Lexer {
    input: Vec<char>,
    current: usize,
    next: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input:&str) -> Lexer{
        todo!("Lexer::new not implemented");
    }
    pub fn read_token(&mut self) -> Token {
        todo!("Lexer::read_token not implemented");
    }
    pub fn next_token(&mut self) -> Token {
        todo!("Lexer::next_token not implemented");
    }
    
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn read_token() {
        let input ="=+(){};";
        let expect_token = vec![
            Token { kind: Tokenkind::Assign, literal: String::from("=") },
            Token { kind: Tokenkind::Plus, literal: String::from("+") },
            Token { kind: Tokenkind::Lparen, literal: String::from("(") },
            Token { kind: Tokenkind::Rparen, literal: String::from(")") },
            Token { kind: Tokenkind::Lbrace, literal: String::from("{") },
            Token { kind: Tokenkind::Rbrace, literal: String::from("}") },
            Token { kind: Tokenkind::Semicolon, literal: String::from(";") },
            Token::new(Tokenkind::Eof, String::from("")), // Eof token
        ];
    }
}