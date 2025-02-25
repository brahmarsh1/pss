use std::collections::HashMap;

/// Defines available transliteration schemes.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TransliterationScheme {
    HarvardKyoto,
    Devanagari,
    Unicode,
}

/// Defines the pitch (Swara) based on Pāṇini's Śikṣā 2.2
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Swara {
    Udaatta,        // High pitch (Pāṇini Śikṣā 2.2 - "udāttānudāttau")
    Anudaatta,      // Low pitch
    Svarita,       // Mixed pitch
}

/// Defines Sama Svara mapping from Vedic Swaras to musical notes
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum SamaSvara {
    Sa, // षड्ज (Shadja) - Corresponds to Svarita
    Re, // ऋषभ (Rishabha) - Corresponds to Anudaatta
    Ga, // गान्धार (Gandhara) - Corresponds to Udaatta
    Ma, // मध्यम (Madhyama) - Corresponds to Svarita
    Pa, // पञ्चम (Panchama) - Corresponds to Svarita
    Dha, // धैवत (Dhaivata) - Corresponds to Anudaatta
    Ni, // निषाद (Nishada) - Corresponds to Udaatta
}

/// Defines vowel duration (Matra) from Pāṇini Śikṣā 2.2
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Matra {
    Hrasva,  // Short vowel
    Diirgha,  // Long vowel
    Pluta,   // Prolonged vowel
}

/// Defines place of articulation (Sthanani) based on Pāṇini Śikṣā 4.8
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Varna {
    pub hk: &'static str,  // Harvard-Kyoto Transliteration
    pub dev: &'static str, // Devanagari script
    pub uni: &'static str, // Unicode representation
    pub swara: Option<Swara>, // Pitch
    pub sama_svara: Option<SamaSvara>, // Sama Svara (Musical Mapping)
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
        sama_svara: Option<SamaSvara>,
        matra: Option<Matra>, 
        sthanani: Option<Sthanani>, 
        prayatna: Option<Prayatna>
    ) -> Self {
        Varna { hk, dev, uni, swara, sama_svara, matra, sthanani, prayatna }
    }
}

/// Provides a mapping of transliterations to `Varna`
pub struct VarnaMap;

impl VarnaMap {
    pub fn get_map() -> HashMap<&'static str, Varna> {
        let mut map = HashMap::new();
        
        let varnas = vec![
            // Swaras (Vowels)
            ("a", "अ", "\u{0905}", Some(Swara::Anudaatta), Some(SamaSvara::Sa), Some(Matra::Hrasva), Some(Sthanani::Kantha), Some(Prayatna::Vivrita)),
            ("aa", "आ", "\u{0906}", Some(Swara::Anudaatta), Some(SamaSvara::Sa), Some(Matra::Diirgha), Some(Sthanani::Kantha), Some(Prayatna::Vivrita)),
            ("i", "इ", "\u{0907}", Some(Swara::Udaatta), Some(SamaSvara::Ga), Some(Matra::Hrasva), Some(Sthanani::Talu), Some(Prayatna::Vivrita)),
            ("u", "उ", "\u{0909}", Some(Swara::Anudaatta), Some(SamaSvara::Re), Some(Matra::Hrasva), Some(Sthanani::Oshtha), Some(Prayatna::Vivrita)),
            
            // Vyanjanas (Consonants - Vargas)
            ("ka", "क", "\u{0915}", None, None, None, Some(Sthanani::Kantha), Some(Prayatna::Sprishta)),
            ("kha", "ख", "\u{0916}", None, None, None, Some(Sthanani::Kantha), Some(Prayatna::Mahaprana)),
            ("ga", "ग", "\u{0917}", None, None, None, Some(Sthanani::Kantha), Some(Prayatna::Sprishta)),
            ("gha", "घ", "\u{0918}", None, None, None, Some(Sthanani::Kantha), Some(Prayatna::Mahaprana)),
            ("nga", "ङ", "\u{0919}", None, None, None, Some(Sthanani::Kantha), Some(Prayatna::Nasika))
        ];
        
        for (hk, dev, uni, swara, sama_svara, matra, sthanani, prayatna) in varnas {
            map.insert(hk, Varna::new(hk, dev, uni, swara, sama_svara, matra, sthanani, prayatna));
        }
        
        map
    }
}
