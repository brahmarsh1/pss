use crate::shiksha::Varna;

/// Defines an Akshara (Syllable) as an array of Varnas
#[derive(Debug, Clone)]
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

        // Extract and unify Swara, Sama Svara, Matra
        let first_swara = varnas[0].swara.clone();
        let first_sama_svara = varnas[0].sama_svara.clone();
        let first_matra = varnas[0].matra.clone();

        // Check for consistency across all Varnas
        let all_swara_match = varnas.iter().all(|v| v.swara == first_swara);
        let all_sama_svara_match = varnas.iter().all(|v| v.sama_svara == first_sama_svara);
        let all_matra_match = varnas.iter().all(|v| v.matra == first_matra);

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
