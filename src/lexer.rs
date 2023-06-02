use crate::token::{Token, TokenType};

pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
    read_position: usize,
    char: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input: input.as_bytes(),
            position: 0,
            read_position: 0,
            char: 0,
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let (r#type, literal) = match self.char {
            b'=' => match self.peak_char() {
                b'=' => {
                    let char = self.char;
                    self.read_char();
                    (TokenType::Eq, u8s_to_string(&[char, self.char]))
                }
                _ => (TokenType::Assign, u8_to_string(self.char)),
            },
            b'+' => (TokenType::Plus, u8_to_string(self.char)),
            b'-' => (TokenType::Minus, u8_to_string(self.char)),
            b'!' => match self.peak_char() {
                b'=' => {
                    let char = self.char;
                    self.read_char();
                    (TokenType::NotEq, u8s_to_string(&[char, self.char]))
                }
                _ => (TokenType::Bang, u8_to_string(self.char)),
            },
            b'/' => (TokenType::Slash, u8_to_string(self.char)),
            b'*' => (TokenType::Asterisk, u8_to_string(self.char)),
            b'<' => (TokenType::Lt, u8_to_string(self.char)),
            b'>' => (TokenType::Gt, u8_to_string(self.char)),
            b';' => (TokenType::Semicolon, u8_to_string(self.char)),
            b'(' => (TokenType::LParen, u8_to_string(self.char)),
            b')' => (TokenType::RParen, u8_to_string(self.char)),
            b',' => (TokenType::Comma, u8_to_string(self.char)),
            b'{' => (TokenType::LBrace, u8_to_string(self.char)),
            b'}' => (TokenType::RBrace, u8_to_string(self.char)),
            b'\0' => (TokenType::Eof, u8_to_string(self.char)),
            _ => {
                if is_letter(self.char) {
                    let identifier = self.read_identifier();
                    return Token::new(TokenType::identifier_token(&identifier), identifier);
                } else if is_digit(self.char) {
                    let number = self.read_number();
                    return Token::new(TokenType::Integer, number);
                } else {
                    (TokenType::Illegal, u8_to_string(self.char))
                }
            }
        };

        self.read_char();

        Token::new(r#type, literal)
    }

    fn peak_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.char = 0;
        } else {
            self.char = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_number(&mut self) -> String {
        let start_position = self.position;

        while is_digit(self.char) {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[start_position..self.position]).to_string()
    }

    fn read_identifier(&mut self) -> String {
        let start_position = self.position;

        while is_letter(self.char) {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[start_position..self.position]).to_string()
    }

    fn skip_whitespace(&mut self) {
        while [b' ', b'\t', b'\n', b'\r'].contains(&self.char) {
            self.read_char();
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token();

        match token.token_type() {
            TokenType::Eof => None,
            _ => Some(token),
        }
    }
}

fn is_digit(char: u8) -> bool {
    char.is_ascii_digit()
}

fn is_letter(char: u8) -> bool {
    char.is_ascii_lowercase() || char.is_ascii_uppercase() || char == b'_'
}

fn u8_to_string(char: u8) -> String {
    u8s_to_string(&[char])
}

fn u8s_to_string(chars: &[u8]) -> String {
    String::from_utf8_lossy(chars).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token_basic() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Plus, "+".to_string()),
            Token::new(TokenType::LParen, "(".to_string()),
            Token::new(TokenType::RParen, ")".to_string()),
            Token::new(TokenType::LBrace, "{".to_string()),
            Token::new(TokenType::RBrace, "}".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
        ];

        for expected in expected_tokens {
            assert_eq!(lexer.next_token(), expected);
        }
    }

    #[test]
    fn test_next_token_small_program() {
        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);

!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9
";

        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Identifier, "five".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Identifier, "ten".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Integer, "10".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Identifier, "add".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Function, "fn".to_string()),
            Token::new(TokenType::LParen, "(".to_string()),
            Token::new(TokenType::Identifier, "x".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Identifier, "y".to_string()),
            Token::new(TokenType::RParen, ")".to_string()),
            Token::new(TokenType::LBrace, "{".to_string()),
            Token::new(TokenType::Identifier, "x".to_string()),
            Token::new(TokenType::Plus, "+".to_string()),
            Token::new(TokenType::Identifier, "y".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::RBrace, "}".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Identifier, "result".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Identifier, "add".to_string()),
            Token::new(TokenType::LParen, "(".to_string()),
            Token::new(TokenType::Identifier, "five".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Identifier, "ten".to_string()),
            Token::new(TokenType::RParen, ")".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Bang, "!".to_string()),
            Token::new(TokenType::Minus, "-".to_string()),
            Token::new(TokenType::Slash, "/".to_string()),
            Token::new(TokenType::Asterisk, "*".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Lt, "<".to_string()),
            Token::new(TokenType::Integer, "10".to_string()),
            Token::new(TokenType::Gt, ">".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::If, "if".to_string()),
            Token::new(TokenType::LParen, "(".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Lt, "<".to_string()),
            Token::new(TokenType::Integer, "10".to_string()),
            Token::new(TokenType::RParen, ")".to_string()),
            Token::new(TokenType::LBrace, "{".to_string()),
            Token::new(TokenType::Return, "return".to_string()),
            Token::new(TokenType::True, "true".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::RBrace, "}".to_string()),
            Token::new(TokenType::Else, "else".to_string()),
            Token::new(TokenType::LBrace, "{".to_string()),
            Token::new(TokenType::Return, "return".to_string()),
            Token::new(TokenType::False, "false".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::RBrace, "}".to_string()),
            Token::new(TokenType::Integer, "10".to_string()),
            Token::new(TokenType::Eq, "==".to_string()),
            Token::new(TokenType::Integer, "10".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Integer, "10".to_string()),
            Token::new(TokenType::NotEq, "!=".to_string()),
            Token::new(TokenType::Integer, "9".to_string()),
            Token::new(TokenType::Eof, "\0".to_string()),
        ];

        for expected in expected_tokens {
            assert_eq!(lexer.next_token(), expected);
        }
    }
}
