#[derive(Debug)]
pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self { statements: vec![] }
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }

    pub fn get_statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
}

#[derive(Debug)]
pub struct LetStatement {
    pub name: Identifier,
    // TODO expressions parsing
    // pub value: Box<dyn Expression>,
}

#[derive(Debug)]
pub struct ReturnStatement {
    // TODO expressions parsing
    // pub return_value: Expression,
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
}

#[derive(Debug)]
pub struct Identifier {
    pub value: String,
}
