use std::fmt::Display;

#[allow(dead_code, unused_variables)]
#[derive(Debug,PartialEq)]
pub struct Token{
    pub kind: Tokenkind,
    pub literal: String,
}

impl Token {
    pub fn new(kind: Tokenkind, literal: String) -> Self {
        Self { kind, literal }
    }
    
}

#[allow(dead_code, unused_variables)]
#[derive(Debug,PartialEq)]
pub enum Tokenkind {
    Illegal,
    Eof,
    Ident,
    Int,
    String,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Eq,
    NotEq,
    Lthan,
    Gthan,

    // Keywords
    Function,
    Let,
    If,
    Else,
    Return,

}

impl Display for Tokenkind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tokenkind::Illegal => write!(f, "Illegal"),
            Tokenkind::Eof => write!(f, "Eof"),
            Tokenkind::Ident => write!(f, "Ident"),
            Tokenkind::Int => write!(f, "Int"),
            Tokenkind::String => write!(f, "String"),
            Tokenkind::Assign => write!(f, "Assign"),
            Tokenkind::Plus => write!(f, "Plus"),
            Tokenkind::Minus => write!(f, "Minus"),
            Tokenkind::Bang => write!(f, "Bang"),
            Tokenkind::Asterisk => write!(f, "Asterisk"),
            Tokenkind::Slash => write!(f, "Slash"),
            Tokenkind::Comma => write!(f, "Comma"),
            Tokenkind::Semicolon => write!(f, "Semicolon"),
            Tokenkind::Lparen => write!(f, "Lparen"),
            Tokenkind::Rparen => write!(f, "Rparen"),
            Tokenkind::Lbrace => write!(f, "Lbrace"),
            Tokenkind::Rbrace => write!(f, "Rbrace"),
            Tokenkind::Eq => write!(f, "Eq"),
            Tokenkind::NotEq => write!(f, "NotEq"),
            Tokenkind::Lthan => write!(f, "Lthan"),
            Tokenkind::Gthan => write!(f, "Gthan"),

            // Keywords
            Tokenkind::Function => write!(f, "Function"),
            Tokenkind::Let => write!(f, "Let"),
            Tokenkind::If => write!(f, "If"),
            Tokenkind::Else => write!(f, "Else"),
            Tokenkind::Return => write!(f, "Return"),
        }
    }
}

impl Tokenkind{
    pub fn lookup_ident(ident: &String) -> Self{
        match ident as &str {
            "fn" => Self::Function,
            "let" => Self::Let,
            "if" => Self::If,
            "else" => Self::Else,
            "return" => Self::Return,
            _ => Self::Ident,
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;

//     impl Display for Token {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Token(kind: {}, literal: {})", self.kind, self.literal)
//     }
// }

    #[test]
    fn test_token_display() {
        let token = Token {
            kind: Tokenkind::Ident,
            literal: String::from("example"),
        };
        assert_eq!(token.kind.to_string(), "Ident");
        assert_eq!(token.literal, "example");
    }
    #[test]
    fn test_tokenkind_display() {
        let token_kind = Tokenkind::Eof;
        assert_eq!(token_kind.to_string(), "Eof");
   }

   
}