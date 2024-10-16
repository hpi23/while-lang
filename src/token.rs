#[derive(Debug, PartialEq, Eq)]

pub enum Token {
    Semicolon,
    NotEq,
    If,
    While,
    Do,
    End,
    Then,
    Zero,
    One,
    Plus,
    Minus,
    Var(u64),
    Assign,
    Eof,
    LeftBracket,
    RightBracket,
    Equal,
    Question,
}
