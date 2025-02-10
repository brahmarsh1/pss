use pss::chandas::{Matra};

fn main() {
    let laghu = Matra::laghu();
    let guru = Matra::guru();

    println!("Laghu Matra: {:?} (1 Kaala)", laghu);
    println!("Guru Matra: {:?} (2 Kaala)", guru);
}

