#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,

    Identifier,
    Integer,

    Assign,
    Plus,
    Minus,
    Bang,
    Asterik,
    Slash,

    Lt,
    Gt,

    Eq,
    NotEq,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl TokenType {
    pub fn identifier_token(identifier: &str) -> Self {
        match identifier {
            "fn" => Self::Function,
            "let" => Self::Let,
            "true" => Self::True,
            "false" => Self::False,
            "if" => Self::If,
            "else" => Self::Else,
            "return" => Self::Return,
            _ => Self::Identifier,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    r#type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(r#type: TokenType, literal: String) -> Self {
        Self { r#type, literal }
    }
}
