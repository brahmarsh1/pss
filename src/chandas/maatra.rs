use crate::chandas::Kaala;

/// `Maatra` represents a single metrical unit (Laghu, Guru, or Pluta).
#[derive(Debug, PartialEq, Eq)]
pub struct Maatra {
    pub length: Kaala,
}

impl Maatra {
    // Creates a new Laghu (1 Kaala unit).
    pub fn laghu() -> Self {
        Maatra { length: Kaala::One }
    

    /// Creates a new Guru (2 Kaala units).
    pub fn guru() -> Self {
        Maatra { length: Kaala::Two }
    }

    /// Creates a new Pluta (3 Kaala units).
    pub fn pluta() -> Self {
        Maatra { length: Kaala::Three }
    }
}
