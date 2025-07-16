use crate::lexericals::tokenizer::{Token, Tokenkind};

#[derive(Debug)]
pub struct Lexer {
    pub input: Vec<char>,
   // pub current: usize,
    pub next: usize,
    pub ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lex = Lexer {
            input: input.chars().collect(),
            //current: 0,
            next: 0,
            ch: '\0',
        };
        lex.read_token();
        lex
    }

    pub fn read_token(&mut self) {
        let current_pos = self.next;
        if current_pos >= self.input.len() {
            self.ch = '\0'; // End of input
        } else {
            self.ch = self.input[current_pos];
        }
        //self.current = current_pos;
        self.next += 1;
    }

    pub fn next_token(&mut self) -> Token {
        //"=+(){},;";
        self.skip_whitespace(); // Skip whitespace characters
        let token = match self.ch {
            '=' => {
                self.read_token(); // Read the next character to check for '=='
                if self.ch == '=' {
                    Token::new(Tokenkind::Eq, "==".to_string())
                }else {
                    Token::new(Tokenkind::Assign, "=".to_string())
                }
            },
            '+' => Token::new(Tokenkind::Plus, self.ch.to_string()),
            '-' => Token::new(Tokenkind::Minus, self.ch.to_string()),
            ',' => Token::new(Tokenkind::Comma, self.ch.to_string()),
            '(' => Token::new(Tokenkind::Lparen, self.ch.to_string()),
            ')' => Token::new(Tokenkind::Rparen, self.ch.to_string()),
            '}' => Token::new(Tokenkind::Rbrace, self.ch.to_string()),
            '{' => Token::new(Tokenkind::Lbrace, self.ch.to_string()),
            ';' => Token::new(Tokenkind::Semicolon, self.ch.to_string()),
            '!' => {
                self.read_token(); // Read the next character to check for '!='
                if self.ch == '=' {
                    Token::new(Tokenkind::NotEq, "!=".to_string())
                } else {
                    Token::new(Tokenkind::Bang, "!".to_string())
                }
            }
            '|' => {
                self.read_token(); // Read the next character to check for '||'
                if self.ch == '|' {
                    Token::new(Tokenkind::OR, "||".to_string())
                } else {
                    Token::new(Tokenkind::Illegal, "|".to_string())
                }
            },
            '*' => Token::new(Tokenkind::Asterisk, self.ch.to_string()),
            '/' => Token::new(Tokenkind::Slash, self.ch.to_string()),
            '%' => Token::new(Tokenkind::Pencentage, self.ch.to_string()),
            '<' => {
                self.read_token(); // Read the next character to check for '<='
                if self.ch == '=' {
                    Token::new(Tokenkind::Lthanoreq, "<=".to_string())
                } else {
                    Token::new(Tokenkind::Lthan, "<".to_string())
                }
              
            },
            '>' =>{
                self.read_token(); // Read the next character to check for '>='
                if self.ch == '=' {
                    Token::new(Tokenkind::Gthanoreq, ">=".to_string())
                } else {
                    Token::new(Tokenkind::Gthan, ">".to_string())
                }
            }, 
            '&' => {
                self.read_token(); // Read the next character to check for '&&'
                if self.ch == '&' {
                    Token::new(Tokenkind::And, "&&".to_string())
                } else {
                    Token::new(Tokenkind::Illegal, "&".to_string())
                }
            }
            '\0' => Token::new(Tokenkind::Eof, String::new()), // End of file token

            _ => {
                return if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let kind = Tokenkind::lookup_ident(&literal);
                    Token::new(kind, literal)
                } else if Lexer::is_digit(self.ch) {
                    let literal = self.read_number();
                    let kind = if literal.contains('.') {
                        Tokenkind::Float // If it contains a dot, treat it as a float
                    } else {
                        Tokenkind::Int // Otherwise, treat it as an integer
                    };
                    Token::new(kind, literal)
                } else {
                    Token::new(Tokenkind::Illegal, self.ch.to_string())
                };
            }
        };
        self.read_token(); // Move to the next character
        token
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_token();
        }
    }

    fn is_digit(ch: char) -> bool {
        ch.is_numeric()
    }

    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }
    fn read_number(&mut self) -> String {
        let mut literal = String::new();
        while Lexer::is_digit(self.ch) {
            literal.push(self.ch);
            self.read_token();
        }

        // Check for a decimal point to handle floats
        // If the next character is a dot, we treat it as a float
        if self.ch == '.' {
            literal.push(self.ch);
            self.read_token();
        }
        while Lexer::is_digit(self.ch) {
            literal.push(self.ch);
            self.read_token();
        }
        literal
    }

    fn read_identifier(&mut self) -> String {
        let mut literal = String::new();
        while Lexer::is_letter(self.ch) {
            literal.push(self.ch);
            self.read_token();
        }
        literal
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn read_token() {
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

        let expect_token = vec![
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
        let mut lexer = Lexer::new(input);
        for (idx, exp_token) in expect_token.iter().enumerate() {
            let recv_token = lexer.next_token();
            assert_eq!(
                exp_token.kind, recv_token.kind,
                "fail to read token at {} expected token {}  found {}",
                idx, exp_token.kind, recv_token.kind
            );
            assert_eq!(
                exp_token.literal, recv_token.literal,
                "fail to read token at {} expectec token {}  found {}",
                idx, exp_token.literal, recv_token.literal
            ); // Move to the next character
        }
    }
    #[test]
    fn test_skip_whitespace() {
        let input = "              let five = 5; \n let ten = 10;";
        let mut lexer = Lexer::new(input);
        let token1 = lexer.next_token();
        assert_eq!(token1.kind, Tokenkind::Let);
        assert_eq!(token1.literal, "let");
    }

    #[test]
    fn test_read_number() {
        let input = "12345 67890 12.34 56.78";
        let mut lexer = Lexer::new(input);
        let token1 = lexer.next_token();
        assert_eq!(token1.kind, Tokenkind::Int);
        assert_eq!(token1.literal, "12345");   
    }
    #[test]
    fn test_read_float() {
        let input = "0.34 56.78";
        let mut lexer = Lexer::new(input);
        let token1 = lexer.next_token();
        assert_eq!(token1.kind, Tokenkind::Float);
        assert_eq!(token1.literal, "0.34");

        let token2 = lexer.next_token();
        assert_eq!(token2.kind, Tokenkind::Float);
        assert_eq!(token2.literal, "56.78");
    }
    #[test]
    fn test_read_keywords() {
        let input = "let fn if else return";
        let mut lexer = Lexer::new(input);
        let token1 = lexer.next_token();
        assert_eq!(token1.kind, Tokenkind::Let);
        assert_eq!(token1.literal, "let");

        let token2 = lexer.next_token();
        assert_eq!(token2.kind, Tokenkind::Function);
        assert_eq!(token2.literal, "fn");

        let token3 = lexer.next_token();
        assert_eq!(token3.kind, Tokenkind::If);
        assert_eq!(token3.literal, "if");

        let token4 = lexer.next_token();
        assert_eq!(token4.kind, Tokenkind::Else);
        assert_eq!(token4.literal, "else");

        let token5 = lexer.next_token();
        assert_eq!(token5.kind, Tokenkind::Return);
        assert_eq!(token5.literal, "return");

    }
    #[test]
    fn test_read_identifiers() {
        let input = "variable_name anotherVariable _privateVar";
        let mut lexer = Lexer::new(input);
        let token1 = lexer.next_token();
        assert_eq!(token1.kind, Tokenkind::Ident);
        assert_eq!(token1.literal, "variable_name");
        let token2 = lexer.next_token();
        assert_eq!(token2.kind, Tokenkind::Ident);
        assert_eq!(token2.literal, "anotherVariable");
        let token3 = lexer.next_token();
        assert_eq!(token3.kind, Tokenkind::Ident);
        assert_eq!(token3.literal, "_privateVar");
    }
    #[test]
    fn test_read_operators() {
        let input = "+ - * / % < > && || ! = == != >= <= ";
        let mut lexer = Lexer::new(input);
        let token1 = lexer.next_token();
        assert_eq!(token1.kind, Tokenkind::Plus);
        assert_eq!(token1.literal, "+");
        let token2 = lexer.next_token();
        assert_eq!(token2.kind, Tokenkind::Minus);
        assert_eq!(token2.literal, "-");
        let token3 = lexer.next_token();
        assert_eq!(token3.kind, Tokenkind::Asterisk);
        assert_eq!(token3.literal, "*");
        let token4 = lexer.next_token();
        assert_eq!(token4.kind, Tokenkind::Slash);
        assert_eq!(token4.literal, "/");
        let token5 = lexer.next_token();
        assert_eq!(token5.kind, Tokenkind::Pencentage);
        assert_eq!(token5.literal, "%");
        let token6 = lexer.next_token();
        assert_eq!(token6.kind, Tokenkind::Lthan);
        assert_eq!(token6.literal, "<");
        let token7 = lexer.next_token();
        assert_eq!(token7.kind, Tokenkind::Gthan);
        assert_eq!(token7.literal, ">");
        let token8 = lexer.next_token();
        assert_eq!(token8.kind, Tokenkind::And);
        assert_eq!(token8.literal, "&&");
        let token9 = lexer.next_token();
        assert_eq!(token9.kind, Tokenkind::OR);
        assert_eq!(token9.literal, "||");
        let token10 = lexer.next_token();
        assert_eq!(token10.kind, Tokenkind::Bang);
        assert_eq!(token10.literal, "!");
        let token11 = lexer.next_token();
        assert_eq!(token11.kind, Tokenkind::Assign);
        assert_eq!(token11.literal, "=");
        let token12 = lexer.next_token();
        assert_eq!(token12.kind, Tokenkind::Eq);
        assert_eq!(token12.literal, "==");
        let token13 = lexer.next_token();
        assert_eq!(token13.kind, Tokenkind::NotEq);
        assert_eq!(token13.literal, "!=");
        let token14 = lexer.next_token();
        assert_eq!(token14.kind, Tokenkind::Gthanoreq);
        assert_eq!(token14.literal, ">=");
        let token15 = lexer.next_token();
        assert_eq!(token15.kind, Tokenkind::Lthanoreq);
        assert_eq!(token15.literal, "<=");
    }
}
