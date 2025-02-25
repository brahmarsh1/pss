use crate::shiksha::Varna;

/// Defines an Akshara (Syllable) as an array of Varnas
#[derive(Debug, Clone)]
pub struct Akshara {
    pub varnas: Vec<Varna>, // An Akshara consists of multiple Varnas
}

impl Akshara {
    /// Creates a new Akshara from a list of Varnas
    pub fn new(varnas: Vec<Varna>) -> Self {
        Akshara { varnas }
    }

    /// Returns the transliteration of the Akshara using Harvard-Kyoto scheme
    pub fn transliterate(&self) -> String {
        self.varnas.iter().map(|v| v.hk).collect::<String>()
    }
}
