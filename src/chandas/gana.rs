use crate::chandas::{Maatra,Kaala};

/// `Gana` represents a grouping of Mātrās forming a word or syllabic unit.
#[derive(Debug, PartialEq, Eq)]
pub struct Gana {
    pub maatras: Vec<Maatra>,
}

impl Gana {
    /// Creates a new `Gana` from a vector of Mātrās.
    pub fn new(maatras: Vec<Maatra>) -> Self {
        Gana { maatras }
    }

    /// Appends a Mātra to the existing `Gana`.
    pub fn push(&mut self, maatra: Maatra) {
        self.maatras.push(maatra);
    }

    /// Returns the total Kaala count of the `Gana`.
    pub fn total_kaala(&self) -> u32 {
        self.maatras.iter().map(|m| match m.length {
            Kaala::One => 1,
            Kaala::Two => 2,
            Kaala::Three => 3,
        }).sum()
    }
}
