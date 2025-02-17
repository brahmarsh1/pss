#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Character(char),
}

pub struct Lexer<'a> {
    input: &'a str,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer instance.
    pub fn new(input: &'a str) -> Self {
        Lexer { input }
    }

    /// Tokenizes the input string into character tokens.
    pub fn tokenize(&self) -> Vec<Token> {
        self.input.chars().map(|c| Token::Character(c)).collect()
    }
}

