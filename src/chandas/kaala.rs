/// `Kaala` represents a unit of metrical time.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Kaala {
    One,   // Laghu (1 Kaala unit)
    Two,   // Guru (2 Kaala units)
    Three, // Pluta (3 Kaala units)
}
