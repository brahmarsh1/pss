use crate::shiksha::{Varna,VarnaMap}; 


/// Represents a tokenized Sanskrit phoneme.
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Varna(Varna),    // Recognized Sanskrit phonetic unit
    Unknown(char),   // Unrecognized character
}

pub struct Lexer<'a> {
    input: &'a str,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer instance.
    pub fn new(input: &'a str) -> Self {
        Lexer { input }
    }

    /// Tokenizes the input string into Sanskrit phonemes (Varna).
    pub fn tokenize(&self) -> Vec<Token> {
        let varna_map = VarnaMap::get_map(); // Get Harvard-Kyoto mappings
        let mut tokens = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = self.input.chars().collect();

        while i < chars.len() {
            let mut found = false;

            // Try matching 3-character sequences (e.g., 'kSa', 'jJa')
            if i + 2 < chars.len() {
                let slice = chars[i..=i+2].iter().collect::<String>();
                if let Some(&varna) = varna_map.get(slice.as_str()) {
                    tokens.push(Token::Varna(varna));
                    i += 3;
                    found = true;
                }
            }

            // Try matching 2-character sequences (e.g., 'kh', 'gh', 'Th')
            if !found && i + 1 < chars.len() {
                let slice = chars[i..=i+1].iter().collect::<String>();
                if let Some(&varna) = varna_map.get(slice.as_str()) {
                    tokens.push(Token::Varna(varna));
                    i += 2;
                    found = true;
                }
            }

            // Try matching single character (fallback)
            if !found {
                let slice = chars[i].to_string();
                if let Some(&varna) = varna_map.get(slice.as_str()) {
                    tokens.push(Token::Varna(varna));
                } else {
                    tokens.push(Token::Unknown(chars[i])); // Handle unknown characters
                }
                i += 1;
            }
        }

        tokens
    }
}
