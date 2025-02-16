/// Represents a Sanskrit phonetic unit (Varna) based on Panini's Shiksha.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Varna {
    Swara(&'static str, &'static str),      // Vowels (Svarāḥ - 21)
    Sparsha(&'static str, &'static str),    // Stops (Sparśa - 25)
    Antastha(&'static str, &'static str),   // Semivowels (Antastha - 4)
    Ushma(&'static str, &'static str),      // Sibilants (Uṣma - 8)
    Anuswara(&'static str, &'static str),   // Anusvāra (ṁ)
    Visarga(&'static str, &'static str),    // Visarga (ḥ)
    Yama(&'static str, &'static str),       // Nasals (Yama - 8)
}

impl Varna {
    /// Returns the Unicode representation of the Varna.
    pub fn unicode(&self) -> &'static str {
        match self {
            Varna::Swara(_, code) | Varna::Sparsha(_, code) | Varna::Antastha(_, code) | Varna::Ushma(_, code) |
            Varna::Anuswara(_, code) | Varna::Visarga(_, code) | Varna::Yama(_, code) => code,
        }
    }
}

/// Represents a Sanskrit phonetic unit with Roman & Devanagari expression.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Akshara {
    pub roman: &'static str,
    pub devanagari: &'static str,
}

impl Akshara {
    /// Creates a new Akshara.
    pub fn new(roman: &'static str, devanagari: &'static str) -> Self {
        Akshara { roman, devanagari }
    }
}

/// Basic Sanskrit Phonetics Library based on Pāṇini's Śikṣā.
pub struct Shiksha;

impl Shiksha {
    /// Returns all Swaras (21 Vowels).
    pub fn swaras() -> Vec<Varna> {
        vec![
            Varna::Swara("अ", "\u{0905}"), Varna::Swara("आ", "\u{0906}"), Varna::Swara("इ", "\u{0907}"), Varna::Swara("ई", "\u{0908}"),
            Varna::Swara("उ", "\u{0909}"), Varna::Swara("ऊ", "\u{090A}"), Varna::Swara("ऋ", "\u{090B}"), Varna::Swara("ॠ", "\u{0960}"),
            Varna::Swara("ऌ", "\u{090C}"), Varna::Swara("ॡ", "\u{0961}"), Varna::Swara("ए", "\u{090F}"), Varna::Swara("ऐ", "\u{0910}"),
            Varna::Swara("ओ", "\u{0913}"), Varna::Swara("औ", "\u{0914}"), Varna::Swara("अं", "\u{0905}\u{0902}"), Varna::Swara("अः", "\u{0905}\u{0903}"),
        ]
    }
    
    /// Returns all Sparsha (Stops - 25).
    pub fn sparshas() -> Vec<Varna> {
        vec![
            Varna::Sparsha("क", "\u{0915}"), Varna::Sparsha("ख", "\u{0916}"), Varna::Sparsha("ग", "\u{0917}"), Varna::Sparsha("घ", "\u{0918}"), Varna::Sparsha("ङ", "\u{0919}"),
            Varna::Sparsha("च", "\u{091A}"), Varna::Sparsha("छ", "\u{091B}"), Varna::Sparsha("ज", "\u{091C}"), Varna::Sparsha("झ", "\u{091D}"), Varna::Sparsha("ञ", "\u{091E}"),
            Varna::Sparsha("ट", "\u{091F}"), Varna::Sparsha("ठ", "\u{0920}"), Varna::Sparsha("ड", "\u{0921}"), Varna::Sparsha("ढ", "\u{0922}"), Varna::Sparsha("ण", "\u{0923}"),
            Varna::Sparsha("त", "\u{0924}"), Varna::Sparsha("थ", "\u{0925}"), Varna::Sparsha("द", "\u{0926}"), Varna::Sparsha("ध", "\u{0927}"), Varna::Sparsha("न", "\u{0928}"),
        ]
    }
}
