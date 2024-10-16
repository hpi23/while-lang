use std::str::Chars;

use crate::token::Token;

pub struct Lexer<'src> {
    input: Chars<'src>,
    current_char: Option<char>,
}

impl<'src> Lexer<'src> {
    pub fn new(input: &'src str) -> Self {
        Lexer {
            input: input.chars(),
            current_char: None,
        }
    }

    pub fn next(&mut self) {
        self.current_char = self.input.next();
    }

    pub fn lex(mut self) -> Vec<Token> {
        let mut tokens = vec![];
        self.next();
        while self.current_char.is_some() {
            tokens.push(self.lex_token());
        }
        tokens.push(Token::Eof);
        tokens
    }

    fn lex_token(&mut self) -> Token {
        loop {
            match self.current_char {
                Some(' ' | '\n' | '\t' | '\r') => self.next(),
                Some('/') => self.skip_line_comment(),
                _ => break,
            }
        }

        let token = match self.current_char {
            None => Token::Eof,
            Some(';') => Token::Semicolon,
            Some('0') => Token::Zero,
            Some('1') => Token::One,
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('[') => Token::LeftBracket,
            Some(']') => Token::RightBracket,
            Some('=') => Token::Equal,
            Some('?') => Token::Question,
            Some(':') => self.make_with_eq(Token::Assign),
            Some('!') => self.make_with_eq(Token::NotEq),

            Some('W') => return self.make_keyword("WHILE", Token::While),
            Some('D') => return self.make_keyword("DO", Token::Do),
            Some('T') => return self.make_keyword("THEN", Token::Then),
            Some('E') => return self.make_keyword("END", Token::End),
            Some('I') => return self.make_keyword("IF", Token::If),

            Some('x') => return self.make_var(),
            char => panic!("Error while lexing token '{char:?}'"),
        };
        self.next();
        token
    }

    fn skip_line_comment(&mut self) {
        self.next();
        match self.current_char {
            Some('/') => self.next(),
            _ => panic!("missing second slash for comment"),
        }
        while !matches!(self.current_char, Some('\n') | None) {
            self.next()
        }
        self.next();
    }

    fn make_with_eq(&mut self, token: Token) -> Token {
        self.next();
        match self.current_char {
            Some('=') => token,
            _ => panic!("Missing equal character after ! or :"),
        }
    }

    fn make_var(&mut self) -> Token {
        self.next();
        if self.current_char == Some('0') {
            self.next();
            return Token::Var(0);
        }
        let mut num: String = String::new();
        while let Some(char) = self.current_char {
            if !char.is_ascii_digit() {
                break;
            }
            num.push(char);
            self.next();
        }

        Token::Var(num.parse().unwrap())
    }

    fn make_keyword(&mut self, text: &str, token: Token) -> Token {
        for char in text.chars() {
            if self.current_char != Some(char) {
                panic!("Encountered incomplete token '{text:?}'")
            }
            self.next();
        }
        token
    }
}
