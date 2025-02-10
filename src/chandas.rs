/// Kaala represents a unit of metrical time
#[derive(Debug, PartialEq, Eq)]
pub enum Kaala {
    One,   // Laghu (1 Kaala unit)
    Two,   // Guru (2 Kaala units)
    Three, // Pluta (3 Kaala units) 
}

/// Matra represents a single metrical unit (Laghu or Guru).
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

    pub fn pluta() -> Self{
        Maatra { length: Kaala::Three }
    }
}

