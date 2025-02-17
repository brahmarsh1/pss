// encoder.rs

use crate::akshara::{Akshara, Svara, Matra};

/// Encoder for Baraha-encoded Vedic pada.
/// This module provides a function to split a pada into its constituent aksharas
/// along with their associated swaras (tones) and chandas (meter: Guru/Laghu).
pub struct Encoder;

impl Encoder {
    /// Encodes a Baraha‑encoded pada into a vector of `Akshara`.
    ///
    /// This function uses a simple heuristic:
    /// - It scans the input string character by character.
    /// - Special marker characters (such as 'q' for anudaatta or '#' for svarita)
    ///   modify the tone of the current akshara but are not added to the output token.
    /// - When a token reaches a certain length (here, 3 characters for demonstration),
    ///   the function finalizes the token as one akshara.
    ///
    /// # Arguments
    ///
    /// * `pada` - A Baraha‑encoded string slice representing the Vedic pada.
    ///
    /// # Example
    ///
    /// Given the pada `"raqSmi"`, the encoder will split it into:
    /// - `"raS"` → with anudaatta (because of the `q` marker), and marked as a Guru (long syllable),
    /// - `"mi"`  → defaulting to Udaatta and marked as Laghu (short syllable).
    ///
    /// ```rust
    /// let aksharas = Encoder::encode("raqSmi");
    /// assert_eq!(aksharas.len(), 2);
    /// assert_eq!(aksharas[0].baraha, "raS");
    /// assert_eq!(aksharas[0].svara, Svara::Anudaatta);
    /// assert_eq!(aksharas[0].matra, Matra::Guru);
    /// assert_eq!(aksharas[1].baraha, "mi");
    /// assert_eq!(aksharas[1].matra, Matra::Laghu);
    /// ```
    pub fn encode(pada: &str) -> Vec<Akshara> {
        let mut aksharas = Vec::new();
        let mut current_token = String::new();
        // Default tone: if no marker is found, assume Udaatta.
        let mut current_svara = Svara::Udaatta;

        let mut chars = pada.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                // The marker 'q' indicates anudaatta; record it without adding to the token.
                'q' => {
                    current_svara = Svara::Anudaatta;
                }
                // The marker '#' could indicate svarita; record it similarly.
                '#' => {
                    current_svara = Svara::Svarita;
                }
                // Otherwise, add the character to the current token.
                _ => {
                    current_token.push(c);
                }
            }

            // Heuristic: for demonstration, we assume an akshara is complete
            // once the current token reaches a length of 3 characters.
            // (This logic should be replaced with proper linguistic rules for Baraha.)
            if current_token.len() >= 3 {
                // Optionally, you could inspect the next character to decide if this token is complete.
                // Here, we simply finalize when the length is reached.
                aksharas.push(Self::finalize_akshara(&current_token, current_svara));
                current_token.clear();
                current_svara = Svara::Udaatta; // reset to default for the next akshara
            }
        }

        // If any token remains, finalize it as well.
        if !current_token.is_empty() {
            aksharas.push(Self::finalize_akshara(&current_token, current_svara));
        }

        aksharas
    }

    /// Finalizes an akshara token by determining its chandas (Guru/Laghu)
    /// and mapping the Baraha token to its corresponding Devanagari representation.
    fn finalize_akshara(token: &str, svara: Svara) -> Akshara {
        // Determine chandas (Guru/Laghu):
        // For this example, if the token ends with an uppercase letter, we treat it as a long syllable (Guru).
        // Otherwise, it is considered short (Laghu).
        let matra = if token.chars().last().unwrap().is_uppercase() {
            Matra::Guru
        } else {
            Matra::Laghu
        };

        // Map the Baraha token to a Devanagari character.
        // This mapping is simplified for demonstration purposes.
        let devanagari = match token {
            "raS" => "र",
            "mi"  => "मि",
            _     => "?",
        };

        Akshara {
            baraha: token,
            devanagari,
            matra,
            svara,
        }
    }
}

