use crate::shiksha::{Varna, Swara, SamaSvara, Matra};

/// Defines an Akshara (Syllable) as an array of Varnas
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Akshara {
    pub varnas: Vec<Varna>, // An Akshara consists of multiple Varnas
    pub swara: Option<String>, // Unified Swara
    pub sama_svara: Option<String>, // Unified Sama Svara
    pub matra: Option<String>, // Unified Matra
}

impl Akshara {
    /// Creates a new Akshara from a list of Varnas
    pub fn new(varnas: Vec<Varna>) -> Option<Self> {
        if varnas.is_empty() {
            return None;
        }

        // Extract first Swara, Sama Svara, and Matra as Strings
        let first_swara = varnas[0].swara.as_ref().map(|s| format!("{:?}", s));
        let first_sama_svara = varnas[0].sama_svara.as_ref().map(|s| format!("{:?}", s));
        let first_matra = varnas[0].matra.as_ref().map(|m| format!("{:?}", m));

        // Check for consistency across all Varnas
        let all_swara_match = varnas.iter().all(|v| v.swara.as_ref().map(|s| format!("{:?}", s)) == first_swara);
        let all_sama_svara_match = varnas.iter().all(|v| v.sama_svara.as_ref().map(|s| format!("{:?}", s)) == first_sama_svara);
        let all_matra_match = varnas.iter().all(|v| v.matra.as_ref().map(|m| format!("{:?}", m)) == first_matra);

        if all_swara_match && all_sama_svara_match && all_matra_match {
            Some(Self {
                varnas,
                swara: first_swara,
                sama_svara: first_sama_svara,
                matra: first_matra,
            })
        } else {
            None // Inconsistent Akshara
        }
    }

    /// Returns the transliteration of the Akshara using Harvard-Kyoto scheme
    pub fn transliterate(&self) -> String {
        self.varnas.iter().map(|v| v.hk.clone()).collect::<String>()
    }
}
