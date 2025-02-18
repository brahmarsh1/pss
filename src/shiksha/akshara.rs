use crate::shiksha::varna::Varna;

/// Represents a Sanskrit syllable or phonetic unit composed of Varnas.
#[derive(Debug, PartialEq, Eq)]
pub struct Akshara {
    pub varnas: Vec<Varna>,
}

impl Akshara {
    /// Creates a new Akshara from a sequence of Varnas.
    pub fn new(varnas: Vec<Varna>) -> Self {
        Akshara { varnas }
    }

    /// Returns the transliteration of the Akshara.
    pub fn transliterate(&self, scheme: super::varna::TransliterationScheme) -> String {
        self.varnas.iter().map(|v| v.transliterate(scheme)).collect()
    }
}
