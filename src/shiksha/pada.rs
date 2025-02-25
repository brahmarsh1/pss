use crate::shiksha::Akshara;

/// Defines a Pada (Word) as an array of Aksharas
#[derive(Debug, Clone)]
pub struct Pada {
    pub aksharas: Vec<Akshara>,
}

impl Pada {
    /// Creates a new Pada from a list of Aksharas
    pub fn new(aksharas: Vec<Akshara>) -> Self {
        Pada { aksharas }
    }

    /// Returns the transliteration of the Pada using Harvard-Kyoto
    pub fn transliterate(&self) -> String {
        self.aksharas.iter().map(|a| a.transliterate()).collect::<Vec<String>>().join(" ")
    }
}
