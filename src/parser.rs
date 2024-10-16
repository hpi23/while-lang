use std::vec::IntoIter;

use crate::{ast::*, Token};

pub struct Parser {
    tokens: IntoIter<Token>,
    current_token: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter(),
            current_token: Token::Eof,
        }
    }

    pub fn parse(mut self) -> Program {
        self.next();
        let program = self.program();
        if self.current_token != Token::Eof {
            panic!("Unexpected token: '{:?}'", self.current_token);
        }
        program
    }

    fn next(&mut self) {
        self.current_token = self.tokens.next().unwrap_or(Token::Eof);
    }

    fn expect(&mut self, to_expect: Token) {
        if self.current_token == to_expect {
            self.next();
        } else {
            panic!(
                "Encountered unexpected token '{:?}', expected '{:?}'",
                self.current_token, to_expect
            )
        }
    }

    fn expect_var(&mut self) -> u64 {
        match self.current_token {
            Token::Var(var) => {
                self.next();
                var
            }
            _ => panic!("Expected variable but got '{:?}'", self.current_token),
        }
    }

    fn program(&mut self) -> Program {
        let mut statements = vec![self.statement()];
        while self.current_token == Token::Semicolon {
            self.next();
            if matches!(self.current_token, Token::Eof | Token::End) {
                break;
            }
            statements.push(self.statement());
        }
        statements
    }

    fn statement(&mut self) -> Statement {
        match self.current_token {
            Token::While => Statement::While(self.while_()),
            Token::If => Statement::If(self.if_()),
            Token::Var(_) => Statement::Assignment(self.assignment()),
            Token::Question => {
                self.next();
                let v = self.expect_var();
                self.expect(Token::Semicolon);
                self.next();
                Statement::Print(v)
            }
            _ => panic!("Invalid statement: '{:?}'", self.current_token),
        }
    }

    fn while_(&mut self) -> While {
        self.next();
        let condition = self.expect_var();
        self.expect(Token::NotEq);
        self.expect(Token::Zero);
        self.expect(Token::Do);
        let program = self.program();
        self.expect(Token::End);

        While { condition, program }
    }

    fn if_(&mut self) -> If {
        self.next();
        let condition_var = self.expect_var();
        self.expect(Token::Equal);
        let condition_const = match &self.current_token {
            Token::Zero => 0,
            Token::One => 1,
            constant => panic!("Unexpected constant in if: '{:?}'", constant),
        };
        self.next();
        self.expect(Token::Then);
        let program = self.program();
        self.expect(Token::End);
        If {
            condition_var,
            condition_const,
            program,
        }
    }

    fn assignment(&mut self) -> Assignment {
        let lhs = self.expect_var();
        self.expect(Token::Assign);

        match self.current_token {
            Token::LeftBracket => {
                let uses_equality = true;
                let rhs_const = 0;
                self.next();
                let rhs_var = self.expect_var();
                self.expect(Token::Equal);
                let rhs_var2 = self.expect_var();
                self.expect(Token::RightBracket);
                Assignment {
                    uses_equality,
                    lhs,
                    rhs_const,
                    rhs_var,
                    rhs_var2,
                }
            }
            _ => {
                let rhs_var = self.expect_var();
                let uses_equality = false;
                let rhs_var2 = 0;
                let rhs_const = match self.current_token {
                    Token::Plus => {
                        self.next();
                        let val = match self.current_token {
                            Token::Zero => 0,
                            Token::One => 1,
                            _ => panic!(
                                "Invalid constant for positive assignment: '{:?}'",
                                self.current_token
                            ),
                        };
                        self.next();
                        val
                    }
                    Token::Minus => {
                        self.next();
                        let val = match self.current_token {
                            Token::Zero => 0,
                            Token::One => -1,
                            _ => panic!(
                                "Invalid constant for negative assignment: '{:?}'",
                                self.current_token
                            ),
                        };
                        self.next();
                        val
                    }
                    _ => 0,
                };
                Assignment {
                    uses_equality,
                    lhs,
                    rhs_var,
                    rhs_var2,
                    rhs_const,
                }
            }
        }
    }
}
