use pss::chandas::{Maatra};

fn main() {
    let laghu = Maatra::laghu();
    let guru = Maatra::guru();
    let pluta = Maatra::pluta();

    println!("Laghu Maatra: {:?} (1 Kaala)", laghu);
    println!("Guru Maatra: {:?} (2 Kaala)", guru);
    println!("Pluta Maatra: {:?} (3 Kaala)", pluta);
}

