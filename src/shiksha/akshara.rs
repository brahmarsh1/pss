use crate::shiksha::{Varna, Swara, SamaSvara};
use crate::chandas::Maatra;

/// Defines an Akshara (Syllable) as an array of Varnas
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Akshara {
    pub varnas: Vec<Varna>, // An Akshara consists of multiple Varnas
    pub swara: Option<String>, // Unified Swara
    pub sama_svara: Option<String>, // Unified Sama Svara
    pub maatra: Maatra, // Syllable duration/weight from chandas
}

impl Akshara {
    /// Creates a new Akshara from a list of Varnas
    pub fn new(varnas: Vec<Varna>) -> Option<Self> {
        if varnas.is_empty() {
            return None;
        }

        // Extract first Swara and Sama Svara as Strings
        let first_swara = varnas[0].swara.as_ref().map(|s| format!("{:?}", s));
        let first_sama_svara = varnas[0].sama_svara.as_ref().map(|s| format!("{:?}", s));

        // Check for consistency across all Varnas
        let all_swara_match = varnas.iter().all(|v| v.swara.as_ref().map(|s| format!("{:?}", s)) == first_swara);
        let all_sama_svara_match = varnas.iter().all(|v| v.sama_svara.as_ref().map(|s| format!("{:?}", s)) == first_sama_svara);

        if all_swara_match && all_sama_svara_match {
            // Determine syllable weight based on Varnas
            let maatra = Self::determine_weight(&varnas);
            
            Some(Self {
                varnas,
                swara: first_swara,
                sama_svara: first_sama_svara,
                maatra,
            })
        } else {
            None // Inconsistent Akshara
        }
    }

    /// Returns the transliteration of the Akshara using Harvard-Kyoto scheme
    pub fn transliterate(&self) -> String {
        self.varnas.iter().map(|v| v.hk.clone()).collect::<String>()
    }

    /// Determines whether an Akshara is Laghu or Guru based on its Varnas
    /// According to classical Sanskrit prosody rules:
    /// 1. A syllable is Guru (heavy) if:
    ///    - It contains a long vowel (diirgha)
    ///    - It contains a short vowel followed by a conjunct consonant
    ///    - It contains anusvara or visarga
    /// 2. A syllable is Laghu (light) if it contains a short vowel followed by at most one consonant
    fn determine_weight(varnas: &[Varna]) -> Maatra {
        // Check if any Varna has Diirgha Matra
        let has_long_vowel = varnas.iter().any(|v| {
            v.matra.as_ref().map_or(false, |m| format!("{:?}", m) == "Diirgha")
        });

        if has_long_vowel {
            return Maatra::guru();
        }

        // Count consonants after the vowel
        let mut consonant_count = 0;
        let mut found_vowel = false;

        for varna in varnas {
            if varna.matra.is_some() {
                found_vowel = true;
            } else if found_vowel {
                consonant_count += 1;
            }
        }

        // If there are multiple consonants after the vowel, it's Guru
        if consonant_count > 1 {
            Maatra::guru()
        } else {
            Maatra::laghu()
        }
    }
}
