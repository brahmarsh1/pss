mod shiksha;

use shiksha::{Shiksha, Varna};

fn main() {
    // Get Sanskrit vowels and consonants
    let vowels = Shiksha::swaras();
    let consonants = Shiksha::sparshas();

    println!("Sanskrit Vowels:");
    for varna in &vowels {
        if let Varna::Swara(devanagari, unicode) = varna {
            println!("{} -> Unicode: {}", devanagari, unicode);
        }
    }

    println!("\nSanskrit Consonants:");
    for varna in &consonants {
        if let Varna::Sparsha(devanagari, unicode) = varna {
            println!("{} -> Unicode: {}", devanagari, unicode);
        }
    }
}
