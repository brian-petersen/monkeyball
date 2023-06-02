use crate::{
    ast::{Identifier, LetStatement, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,

    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Self {
            lexer,
            current_token,
            peek_token,
        }
    }

    pub fn parse_program(mut self) -> Program {
        let mut program = Program::new();

        while self.current_token.get_type() != &TokenType::Eof {
            if let Some(statement) = self.parse_statement() {
                program.add_statement(statement);
            }

            self.next_token();
        }

        program
    }

    fn current_token_is(&self, r#type: TokenType) -> bool {
        self.current_token.get_type() == &r#type
    }

    fn expect_peek(&mut self, r#type: TokenType) -> bool {
        if self.peek_token_is(r#type) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        if !self.expect_peek(TokenType::Identifier) {
            return None;
        }

        let name = Identifier {
            value: self.current_token.get_literal().clone(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        // TODO expressions parsing
        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(LetStatement { name })
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.get_type() {
            &TokenType::Let => self.parse_let_statement().map(Statement::LetStatement),
            _ => None,
        }
    }

    fn peek_token_is(&self, r#type: TokenType) -> bool {
        self.peek_token.get_type() == &r#type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token_basic() {
        let input = "let x = 5;
let y = 10;
let foobar = 838383;
";
        let lexer = Lexer::new(input);
        let program = Parser::new(lexer).parse_program();

        println!("{:#?}", program);

        assert!(false);
    }
}
