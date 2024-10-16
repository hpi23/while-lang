pub type Program = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    While(While),
    Assignment(Assignment),
    Print(u64),
    If(If),
}

#[derive(Debug)]
pub struct While {
    pub condition: u64,
    pub program: Program,
}

#[derive(Debug)]
pub struct If {
    pub condition_var: u64,
    pub condition_const: i8,
    pub program: Program,
}

#[derive(Debug)]
pub struct Assignment {
    pub uses_equality: bool,
    pub lhs: u64,
    pub rhs_var: u64,
    pub rhs_var2: u64,
    pub rhs_const: i8,
}

#[derive(Debug)]
pub struct Print {
    pub var: u64,
}
