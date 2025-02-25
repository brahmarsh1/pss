use crate::shiksha::{Varna, VarnaMap, Akshara};

/// Represents a tokenized Sanskrit phoneme.
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Akshara(Akshara), // Recognized Sanskrit syllable (one or more Varnas)
    Unknown(char),    // Unrecognized character
}

pub struct Lexer<'a> {
    input: &'a str,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer instance.
    pub fn new(input: &'a str) -> Self {
        Lexer { input }
    }

    /// Tokenizes the input string into Sanskrit phonetic syllables (Aksharas).
    pub fn tokenize(&self) -> Vec<Token> {
        let varna_map = VarnaMap::get_map(); // Get Harvard-Kyoto mappings
        let mut tokens = Vec::new();
        let mut current_akshara = Vec::new(); // Collects Varnas for an Akshara
        let chars: Vec<char> = self.input.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let mut found = false;

            // Try matching 3-character sequences (e.g., 'kSa', 'jJa')
            if i + 2 < chars.len() {
                let slice = chars[i..=i+2].iter().collect::<String>();
                if let Some(&varna) = varna_map.get(slice.as_str()) {
                    current_akshara.push(varna);
                    i += 3;
                    found = true;
                }
            }

            // Try matching 2-character sequences (e.g., 'kh', 'gh', 'Th')
            if !found && i + 1 < chars.len() {
                let slice = chars[i..=i+1].iter().collect::<String>();
                if let Some(&varna) = varna_map.get(slice.as_str()) {
                    current_akshara.push(varna);
                    i += 2;
                    found = true;
                }
            }

            // Try matching single character (fallback)
            if !found {
                let slice = chars[i].to_string();
                if let Some(&varna) = varna_map.get(slice.as_str()) {
                    current_akshara.push(varna);
                } else {
                    // If we encounter an unknown character, push the last Akshara and reset
                    if !current_akshara.is_empty() {
                        tokens.push(Token::Akshara(Akshara::new(current_akshara.clone()).unwrap()));
                        current_akshara.clear();
                    }
                    tokens.push(Token::Unknown(chars[i])); // Handle unknown characters
                }
                i += 1;
            }

            // If the current Varna is a vowel, finalize the Akshara
            if let Some(last_varna) = current_akshara.last() {
                if last_varna.swara.is_some() {
                    tokens.push(Token::Akshara(Akshara::new(current_akshara.clone()).unwrap()));
                    current_akshara.clear();
                }
            }
        }

        // Push any remaining Akshara at the end
        if !current_akshara.is_empty() {
            tokens.push(Token::Akshara(Akshara::new(current_akshara).unwrap()));
        }

        tokens
    }
}
