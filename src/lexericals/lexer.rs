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
        if current_pos >= self.input.len() {
            self.ch = '\0'; // End of input
        }else{
            self.ch = self.input[current_pos];          
        }
        self.current = current_pos;
        self.next += 1;
    }

    pub fn next_token(&mut self) -> Token {
            //"=+(){},;";
        let token = match self.ch {
            '=' => Token::new(Tokenkind::Assign, self.ch.to_string()),
            '+' => Token::new(Tokenkind::Plus, self.ch.to_string()),
            '-' => Token::new(Tokenkind::Minus, self.ch.to_string()),
            ',' => Token::new(Tokenkind::Comma, self.ch.to_string()),
            '(' => Token::new(Tokenkind::Lparen, self.ch.to_string()),
            ')' => Token::new(Tokenkind::Rparen, self.ch.to_string()),
            '}' => Token::new(Tokenkind::Rbrace, self.ch.to_string()),
            '{' => Token::new(Tokenkind::Lbrace, self.ch.to_string()),
            ';' => Token::new(Tokenkind::Semicolon, self.ch.to_string()),
            '\0' => Token::new(Tokenkind::Eof, String::new()), // End of file token

            _ => Token::new(Tokenkind::Illegal, self.ch.to_string()),


        
        };
        self.read_token(); // Move to the next character
        token
    }
  
    
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn read_token() {
        let input ="=+(){},;";
        let expect_token = vec![
            Token::new (Tokenkind::Assign,String::from("=")),
            Token::new (Tokenkind::Plus, String::from("+")),
            Token::new (Tokenkind::Lparen, String::from("(")),
            Token::new (Tokenkind::Rparen, String::from(")")),
            Token::new (Tokenkind::Lbrace, String::from("{")),
            Token::new (Tokenkind::Rbrace, String::from("}")),
            Token::new (Tokenkind::Comma, String::from(",")),
            Token::new (Tokenkind::Semicolon,String::from(";")),
            Token::new (Tokenkind::Eof, String::new()), // Eof token
        ];
        let mut lexer = Lexer::new(input);
        for (idx,exp_token) in expect_token.iter().enumerate() {
            let recv_token = lexer.next_token();
            assert_eq!(exp_token.kind,recv_token.kind,
                "fail to read token at {} expected token {}  found {}",idx,exp_token.kind,recv_token.kind);
            assert_eq!(exp_token.literal,recv_token.literal,
                "fail to read token at {} expectec token {}  found {}",idx,exp_token.literal,recv_token.literal);  // Move to the next character
        }
    }
}