use crate::shiksha::{Akshara};

/// Defines a Sutra (Sequence of Aksharas without inherent meaning)
#[derive(Debug, Clone)]
pub struct Sutra {
    pub aksharas: Vec<Akshara>, // A Sutra consists of multiple Aksharas
}

impl Sutra {
    /// Creates a new Sutra from a list of Aksharas
    pub fn new(aksharas: Vec<Akshara>) -> Self {
        Sutra { aksharas }
    }

    /// Returns the transliteration of the Sutra using Harvard-Kyoto scheme
    pub fn transliterate(&self) -> String {
        self.aksharas.iter().map(|a| a.transliterate()).collect::<Vec<String>>().join("")
    }
}
