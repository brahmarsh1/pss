use std::collections::HashMap;

/// Defines available transliteration schemes.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TransliterationScheme {
    HarvardKyoto,
    Devanagari,
    Unicode,
}

/// Represents a Sanskrit phonetic unit (Varna) based on Panini's Shiksha.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Varna {
    pub hk: &'static str,  // Harvard-Kyoto Transliteration
    pub dev: &'static str, // Devanagari script
    pub uni: &'static str, // Unicode representation
}

impl Varna {
    /// Creates a new Varna with transliteration mappings.
    pub const fn new(hk: &'static str, dev: &'static str, uni: &'static str) -> Self {
        Varna { hk, dev, uni }
    }

    /// Returns the representation based on the selected transliteration scheme.
    pub fn transliterate(&self, scheme: TransliterationScheme) -> &'static str {
        match scheme {
            TransliterationScheme::HarvardKyoto => self.hk,
            TransliterationScheme::Devanagari => self.dev,
            TransliterationScheme::Unicode => self.uni,
        }
    }
}

/// Provides a lookup for Varna from transliterations.
pub struct VarnaMap;

impl VarnaMap {
    /// Returns a mapping of Harvard-Kyoto transliterations to `Varna`.
    pub fn get_map() -> HashMap<&'static str, Varna> {
        let mut map = HashMap::new();

        let swaras = vec![
            Varna::new("a", "अ", "\u{0905}"),
            Varna::new("A", "आ", "\u{0906}"),
            Varna::new("i", "इ", "\u{0907}"),
            Varna::new("I", "ई", "\u{0908}"),
            Varna::new("u", "उ", "\u{0909}"),
            Varna::new("U", "ऊ", "\u{090A}"),
            Varna::new("R", "ऋ", "\u{090B}"),
            Varna::new("RR", "ॠ", "\u{0960}"),
            Varna::new("L", "ऌ", "\u{090C}"),
            Varna::new("LL", "ॡ", "\u{0961}"),
            Varna::new("e", "ए", "\u{090F}"),
            Varna::new("ai", "ऐ", "\u{0910}"),
            Varna::new("o", "ओ", "\u{0913}"),
            Varna::new("au", "औ", "\u{0914}"),
            Varna::new("aM", "अं", "\u{0905}\u{0902}"),
            Varna::new("aH", "अः", "\u{0905}\u{0903}"),
        ];

        let sparshas = vec![
            Varna::new("k", "क", "\u{0915}"),
            Varna::new("kh", "ख", "\u{0916}"),
            Varna::new("g", "ग", "\u{0917}"),
            Varna::new("gh", "घ", "\u{0918}"),
            Varna::new("G", "ङ", "\u{0919}"),
            Varna::new("c", "च", "\u{091A}"),
            Varna::new("ch", "छ", "\u{091B}"),
            Varna::new("j", "ज", "\u{091C}"),
            Varna::new("jh", "झ", "\u{091D}"),
            Varna::new("J", "ञ", "\u{091E}"),
            Varna::new("T", "ट", "\u{091F}"),
            Varna::new("Th", "ठ", "\u{0920}"),
            Varna::new("D", "ड", "\u{0921}"),
            Varna::new("Dh", "ढ", "\u{0922}"),
            Varna::new("N", "ण", "\u{0923}"),
            Varna::new("t", "त", "\u{0924}"),
            Varna::new("th", "थ", "\u{0925}"),
            Varna::new("d", "द", "\u{0926}"),
            Varna::new("dh", "ध", "\u{0927}"),
            Varna::new("n", "न", "\u{0928}"),
        ];

        // Insert all varnas into the map
        for varna in swaras.into_iter().chain(sparshas.into_iter()) {
            map.insert(varna.hk, varna);
        }

        map
    }
}
