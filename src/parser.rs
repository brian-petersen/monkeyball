use crate::{
    ast::{Identifier, LetStatement, Program, ReturnStatement, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    current_token: Token,
    errors: Vec<String>,
    lexer: Lexer<'a>,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Self {
            current_token,
            errors: vec![],
            lexer,
            peek_token,
        }
    }

    pub fn parse_program(mut self) -> (Program, Vec<String>) {
        let mut program = Program::new();

        while self.current_token.get_type() != &TokenType::Eof {
            if let Some(statement) = self.parse_statement() {
                program.add_statement(statement);
            }

            self.next_token();
        }

        (program, self.errors)
    }

    fn current_token_is(&self, r#type: TokenType) -> bool {
        self.current_token.get_type() == &r#type
    }

    fn expect_peek(&mut self, expected_type: TokenType) -> bool {
        if self.peek_token_is(&expected_type) {
            self.next_token();
            true
        } else {
            self.peek_error(&expected_type);
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

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        self.next_token();

        // TODO expressions parsing
        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(ReturnStatement {})
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match *self.current_token.get_type() {
            TokenType::Let => self.parse_let_statement().map(Statement::LetStatement),
            TokenType::Return => self
                .parse_return_statement()
                .map(Statement::ReturnStatement),
            _ => None,
        }
    }

    fn peek_error(&mut self, expected_token: &TokenType) {
        self.errors.push(format!(
            "expected next token to be {:?}, got {:?} instead",
            expected_token,
            self.peek_token.get_type()
        ));
    }

    fn peek_token_is(&self, r#type: &TokenType) -> bool {
        self.peek_token.get_type() == r#type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program_errors() {
        let input = "let x 5;
let = 10;
let 838383;
";
        let lexer = Lexer::new(input);
        let (program, errors) = Parser::new(lexer).parse_program();

        assert!(program.get_statements().is_empty());

        assert_eq!(
            errors,
            [
                "expected next token to be Assign, got Integer instead",
                "expected next token to be Identifier, got Assign instead",
                "expected next token to be Identifier, got Integer instead",
            ]
        );
    }

    #[test]
    fn test_let_statements() {
        let input = "let x = 5;
let y = 10;
let foobar = 838383;
";
        let lexer = Lexer::new(input);
        let (program, errors) = Parser::new(lexer).parse_program();

        assert!(errors.is_empty());

        let ids: Vec<String> = program
            .get_statements()
            .iter()
            .filter_map(|s| match s {
                Statement::LetStatement(LetStatement { name }) => Some(name),
                statement => panic!("Expected a LetStatement, got {:?}", statement),
            })
            .map(|i| i.value.to_string())
            .collect();

        assert_eq!(ids, ["x", "y", "foobar"]);
    }

    #[test]
    fn test_return_statements() {
        let input = "return 5;
return 10;
return 993322;
";
        let lexer = Lexer::new(input);
        let (program, errors) = Parser::new(lexer).parse_program();

        assert!(errors.is_empty());

        let todo: Vec<()> = program
            .get_statements()
            .iter()
            .filter_map(|s| match s {
                // TODO expressions parsing
                Statement::ReturnStatement(ReturnStatement {}) => Some(()),
                statement => panic!("Expected a ReturnStatement, got {:?}", statement),
            })
            .collect();

        assert_eq!(todo.len(), 3);
    }
}
