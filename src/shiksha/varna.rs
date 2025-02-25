use std::collections::HashMap;

/// Defines available transliteration schemes.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TransliterationScheme {
    HarvardKyoto,
    Devanagari,
    Unicode,
}

/// Defines the pitch (Swara) based on Pāṇini's Śikṣā 2.2
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Swara {
    Udaatta,        // High pitch (Pāṇini Śikṣā 2.2 - "udāttānudāttau")
    Anudaatta,      // Low pitch
    Svarita,       // Mixed pitch
}

/// Defines vowel duration (Matra) from Pāṇini Śikṣā 2.2
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Matra {
    Hrasva,  // Short vowel
    Diirgha,  // Long vowel
    Pluta,   // Prolonged vowel
}

/// Defines place of articulation (Sthanani) based on Pāṇini Śikṣā 4.8
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Sthanani {
    Uras,   // उरः (Chest)
    Kantha, // कण्ठः (Throat)
    Murdha, // मूर्धा (Head)
    Jihvamula, // जिह्वामूल (Tongue Root)
    Danta,  // दन्त (Teeth)
    Nasika, // नासिका (Nose)
    Oshtha, // ओष्ठ (Lips)
    Talu,   // तालु (Palate)
}

/// Defines articulation effort (Prayatna) based on Pāṇini Śikṣā 2.1
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Prayatna {
    Sprishta,   // Contact (Plosive)
    IshatSparsha, // Slight contact (Fricative)
    Vivrita,   // Open (Vowel)
    Samvruta,   // Semi-closed
    Alpaprana,  // Light aspiration
    Mahaprana,  // Strong aspiration
    Nasika,     // Nasalized articulation
    Anunasika,  // Semi-nasalized articulation
}

/// Represents a complete Sanskrit phonetic unit (Varna) with phonetic attributes.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Varna {
    pub hk: &'static str,  // Harvard-Kyoto Transliteration
    pub dev: &'static str, // Devanagari script
    pub uni: &'static str, // Unicode representation
    pub swara: Option<Swara>, // Pitch
    pub matra: Option<Matra>, // Duration
    pub sthanani: Option<Sthanani>, // Place of articulation
    pub prayatna: Option<Prayatna>, // Effort of articulation
}

impl Varna {
    /// Creates a new Varna with extended phonetic properties.
    pub const fn new(
        hk: &'static str, 
        dev: &'static str, 
        uni: &'static str, 
        swara: Option<Swara>, 
        matra: Option<Matra>, 
        sthanani: Option<Sthanani>, 
        prayatna: Option<Prayatna>
    ) -> Self {
        Varna { hk, dev, uni, swara, matra, sthanani, prayatna }
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

/// Varna Mapping with references to Pāṇini's Śikṣā
pub struct VarnaMap;

impl VarnaMap {
    /// Returns an updated mapping of Harvard-Kyoto transliterations to `Varna` with phonetic details.
    pub fn get_map() -> HashMap<&'static str, Varna> {
        let mut map = HashMap::new();

        // Full set of Varnas from Pāṇini Śikṣā
        let varnas = vec![
            // Swaras (Vowels)
            Varna::new("a", "अ", "\u{0905}", Some(Swara::Anudaatta), Some(Matra::Hrasva), Some(Sthanani::Kantha), Some(Prayatna::Vivrita)),
            Varna::new("aa", "आ", "\u{0906}", Some(Swara::Anudaatta), Some(Matra::Diirgha), Some(Sthanani::Kantha), Some(Prayatna::Vivrita)),
            Varna::new("i", "इ", "\u{0907}", Some(Swara::Udaatta), Some(Matra::Hrasva), Some(Sthanani::Talu), Some(Prayatna::Vivrita)),
            Varna::new("ii", "ई", "\u{0908}", Some(Swara::Udaatta), Some(Matra::Diirgha), Some(Sthanani::Talu), Some(Prayatna::Vivrita)),
            Varna::new("u", "उ", "\u{0909}", Some(Swara::Anudaatta), Some(Matra::Hrasva), Some(Sthanani::Oshtha), Some(Prayatna::Vivrita)),
            Varna::new("uu", "ऊ", "\u{090A}", Some(Swara::Anudaatta), Some(Matra::Diirgha), Some(Sthanani::Oshtha), Some(Prayatna::Vivrita)),
            
            // Vyanjanas (Consonants)
            Varna::new("k", "क", "\u{0915}", None, None, Some(Sthanani::Kantha), Some(Prayatna::Sprishta)),
            Varna::new("kh", "ख", "\u{0916}", None, None, Some(Sthanani::Kantha), Some(Prayatna::Mahaprana)),
            Varna::new("g", "ग", "\u{0917}", None, None, Some(Sthanani::Kantha), Some(Prayatna::Sprishta)),
            Varna::new("gh", "घ", "\u{0918}", None, None, Some(Sthanani::Kantha), Some(Prayatna::Mahaprana)),
            Varna::new("ng", "ङ", "\u{0919}", None, None, Some(Sthanani::Kantha), Some(Prayatna::Nasika)),
        ];

        // Insert all varnas into the map
        for varna in varnas {
            map.insert(varna.hk, varna);
        }

        map
    }
}
