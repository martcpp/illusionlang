use super::tokenizer::{Token,Tokenkind};

#[derive(Debug)]
pub struct Lexer {
    pub input: Vec<char>,
    pub current: usize,
    pub next: usize,
    pub ch: char,
}

impl Lexer {
    pub fn new(input:&str) -> Lexer{
        let mut lex = Lexer {
            input: input.chars().collect(),
            current: 0,
            next: 0,
            ch: '\0',
        };
        lex.read_token();
        lex
    }

    pub fn read_token(&mut self){
        let current_pos = self.next;
        if self.current >= self.input.len() {
            self.ch = '\0'; // End of input
        }else{
            self.ch = self.input[current_pos];
            self.current = current_pos;
            self.next += 1;

        }
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