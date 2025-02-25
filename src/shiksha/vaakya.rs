use crate::shiksha::Pada;

/// Defines a Vaakya (Sentence) as an array of Padas
#[derive(Debug, Clone)]
pub struct Vaakya {
    pub padas: Vec<Pada>, // A Vaakya consists of multiple Padas
}

impl Vaakya {
    /// Creates a new Vaakya from a list of Padas
    pub fn new(padas: Vec<Pada>) -> Self {
        Vaakya { padas }
    }

    /// Returns the transliteration of the Vaakya using Harvard-Kyoto scheme
    pub fn transliterate(&self) -> String {
        self.padas.iter().map(|p| p.transliterate()).collect::<Vec<String>>().join(" ")
    }
}
