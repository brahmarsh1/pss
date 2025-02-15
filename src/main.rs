use pss::chandas::{Maatra, Gana};

fn main() {
    let laghu = Maatra::laghu();
    let guru = Maatra::guru();
    let pluta = Maatra::pluta();

    println!("Laghu Maatra: {:?} (1 Kaala)", laghu);
    println!("Guru Maatra: {:?} (2 Kaala)", guru);
    println!("Pluta Maatra: {:?} (3 Kaala)", pluta);

    // Create a Sabda consisting of a Laghu followed by a Guru
    let Gana = Gana::new(vec![Maatra::laghu(), Maatra::guru()]);
    
    println!("Gana: {:?}", Gana);
    println!("Total Kaala: {}", Gana.total_kaala());
}

