use crate::ast::*;

pub struct Interpreter {
    variables: Vec<u64>,
}

impl Interpreter {
    pub fn new(start_variables: &[u64]) -> Self {
        let mut variables = vec![0];
        variables.extend_from_slice(start_variables);
        Self { variables }
    }

    pub fn run(mut self, tree: Program) -> u64 {
        self.visit_program(&tree);
        self.variables[0]
    }

    fn get_var(&self, index: u64) -> u64 {
        self.variables.get(index as usize).copied().unwrap_or(0)
    }

    fn set_var(&mut self, index: u64, value: u64) {
        match self.variables.get_mut(index as usize) {
            Some(var) => *var = value,
            None => {
                self.variables.resize(index as usize, 0);
                self.variables.push(value);
            }
        }
    }

    fn visit_program(&mut self, node: &Program) {
        for statement in node {
            self.visit_statement(statement);
        }
    }

    fn visit_statement(&mut self, node: &Statement) {
        match node {
            Statement::While(node) => self.visit_while(node),
            Statement::Assignment(node) => self.visit_assignment(node),
            Statement::If(node) => self.visit_if(node),
            Statement::Print(var) => {
                let value = self.get_var(*var);
                println!("{value}");
            }
        }
    }

    fn visit_while(&mut self, node: &While) {
        while self.get_var(node.condition) != 0 {
            self.visit_program(&node.program);
        }
    }

    fn visit_if(&mut self, node: &If) {
        if self.get_var(node.condition_var)
            == match node.condition_const {
                0 => 0,
                _ => 1,
            }
        {
            self.visit_program(&node.program);
        }
    }

    fn visit_assignment(&mut self, node: &Assignment) {
        self.set_var(
            node.lhs,
            if node.uses_equality {
                if self.get_var(node.rhs_var) == self.get_var(node.rhs_var2) {
                    1
                } else {
                    0
                }
            } else {
                match node.rhs_const {
                    -1 => self.get_var(node.rhs_var).saturating_sub(1),
                    val => self.get_var(node.rhs_var).saturating_add(val as u64),
                }
            },
        );
    }
}
