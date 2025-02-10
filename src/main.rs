use pss::chandas::{Maatra, Sabda};

fn main() {
    let laghu = Maatra::laghu();
    let guru = Maatra::guru();
    let pluta = Maatra::pluta();

    println!("Laghu Maatra: {:?} (1 Kaala)", laghu);
    println!("Guru Maatra: {:?} (2 Kaala)", guru);
    println!("Pluta Maatra: {:?} (3 Kaala)", pluta);

    // Create a Sabda consisting of a Laghu followed by a Guru
    let sabda = Sabda::new(vec![Maatra::laghu(), Maatra::guru()]);
    
    println!("Sabda: {:?}", sabda);
    println!("Total Kaala: {}", sabda.total_kaala());
}

