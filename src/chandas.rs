/// Kaala represents a unit of metrical time in Sanskrit Chandas.
#[derive(Debug, PartialEq, Eq)]
pub enum Kaala {
    One,   // Laghu (1 Kaala unit)
    Two,   // Guru (2 Kaala units)
}

/// Matra represents a single metrical unit (Laghu or Guru).
#[derive(Debug, PartialEq, Eq)]
pub struct Matra {
    pub length: Kaala,
}

impl Matra {
    /// Creates a new Laghu (1 Kaala unit).
    pub fn laghu() -> Self {
        Matra { length: Kaala::One }
    }

    /// Creates a new Guru (2 Kaala units).
    pub fn guru() -> Self {
        Matra { length: Kaala::Two }
    }
}

