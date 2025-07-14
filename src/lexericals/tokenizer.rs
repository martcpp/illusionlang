use std::fmt::Display;

#[allow(dead_code, unused_variables)]
#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: Tokenkind,
    pub literal: String,
}

impl Token {
    pub fn new(kind: Tokenkind, literal: String) -> Self {
        Self { kind, literal }
    }
}

#[allow(dead_code, unused_variables)]
#[derive(Debug, PartialEq)]
pub enum Tokenkind {
    // Illegal and End of File and identifiers
    Illegal,
    Eof,
    Ident,
    

    // Types
    Int,
    Float,
    String,
    Char,

    // mathematical Operators
    Plus,
    Minus,
    Asterisk,
    Slash,
    Pencentage,

    // special Characters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Assign,

    // comparison Operators
    Eq,
    NotEq,
    Lthan,
    Gthan,
    OR,
    And,
    Bang,

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
            // Illegal and End of File and identifiers
            Tokenkind::Illegal => write!(f, "Illegal"),
            Tokenkind::Eof => write!(f, "Eof"),
            Tokenkind::Ident => write!(f, "Ident"),
            

            // Types
            Tokenkind::Int => write!(f, "Int"),
            Tokenkind::Float => write!(f, "Float"),
            Tokenkind::String => write!(f, "String"),
            Tokenkind::Char => write!(f, "Char"),

            

            // mathematical Operators
            Tokenkind::Plus => write!(f, "Plus"),
            Tokenkind::Minus => write!(f, "Minus"),
            Tokenkind::Asterisk => write!(f, "Asterisk"),
            Tokenkind::Slash => write!(f, "Slash"),
            Tokenkind::Pencentage => write!(f, "Percentage"),

            // special Characters
            Tokenkind::Comma => write!(f, "Comma"),
            Tokenkind::Semicolon => write!(f, "Semicolon"),
            Tokenkind::Lparen => write!(f, "Lparen"),
            Tokenkind::Rparen => write!(f, "Rparen"),
            Tokenkind::Lbrace => write!(f, "Lbrace"),
            Tokenkind::Rbrace => write!(f, "Rbrace"),
            Tokenkind::Assign => write!(f, "Assign"),

            // equality Operators
            Tokenkind::Eq => write!(f, "Eq"),
            Tokenkind::NotEq => write!(f, "NotEq"),
            Tokenkind::Lthan => write!(f, "Lthan"),
            Tokenkind::Gthan => write!(f, "Gthan"),
            Tokenkind::OR => write!(f, "OR"),
            Tokenkind::And => write!(f, "And"),
            Tokenkind::Bang => write!(f, "Bang"),
            
            // Keywords
            Tokenkind::Function => write!(f, "Function"),
            Tokenkind::Let => write!(f, "Let"),
            Tokenkind::If => write!(f, "If"),
            Tokenkind::Else => write!(f, "Else"),
            Tokenkind::Return => write!(f, "Return"),
        }
    }
}

impl Tokenkind {
    pub fn lookup_ident(ident: &str) -> Self {
        match ident{
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
mod test {
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
