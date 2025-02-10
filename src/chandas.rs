/// Kaala represents a unit of metrical time.
#[derive(Debug, PartialEq, Eq)]
pub enum Kaala {
    One,   // Laghu (1 Kaala unit)
    Two,   // Guru (2 Kaala units)
    Three, // Pluta (3 Kaala units)
}

/// Maatra represents a single metrical unit (Laghu, Guru, or Pluta).
#[derive(Debug, PartialEq, Eq)]
pub struct Maatra {
    pub length: Kaala,
}

impl Maatra {
    /// Creates a new Laghu (1 Kaala unit).
    pub fn laghu() -> Self {
        Maatra { length: Kaala::One }
    }

    /// Creates a new Guru (2 Kaala units).
    pub fn guru() -> Self {
        Maatra { length: Kaala::Two }
    }

    /// Creates a new Pluta (3 Kaala units).
    pub fn pluta() -> Self {
        Maatra { length: Kaala::Three }
    }
}

/// Sabda represents a grouping of Mātrās forming a word or syllabic unit.
#[derive(Debug, PartialEq, Eq)]
pub struct Sabda {
    pub maatras: Vec<Maatra>,
}

impl Sabda {
    /// Creates a new Sabda from a vector of Mātrās.
    pub fn new(maatras: Vec<Maatra>) -> Self {
        Sabda { maatras }
    }

    /// Appends a Mātra to the existing Sabda.
    pub fn push(&mut self, maatra: Maatra) {
        self.maatras.push(maatra);
    }

    /// Returns the total Kaala count of the Sabda.
    pub fn total_kaala(&self) -> u32 {
        self.maatras.iter().map(|m| {
            match m.length {
                Kaala::One => 1,
                Kaala::Two => 2,
                Kaala::Three => 3,
            }
        }).sum()
    }
}

